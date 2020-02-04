// websocketcommunication.rs
//! module that cares about WebSocket communication

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::*;

use mem6_common::*;

use unwrap::unwrap;
use js_sys::Reflect;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{ErrorEvent, WebSocket};
use gloo_timers::future::TimeoutFuture;
//endregion

//the location_href is not consumed in this function and Clippy wants a reference instead a value
//but I don't want references, because they have the lifetime problem.
#[allow(clippy::needless_pass_by_value)]
///setup WebSocket connection
pub fn setup_ws_connection(
    location_href: String,
    client_ws_id: usize,
    players_ws_uid: String,
) -> WebSocket {
    //web-sys has WebSocket for Rust exactly like JavaScript has¸
    //location_href comes in this format  http://localhost:4000/
    let mut loc_href = location_href
        .replace("http://", "ws://")
        .replace("https://", "wss://");
    //Only for debugging in the development environment
    //let mut loc_href = String::from("ws://192.168.1.57:80/");
    logmod::debug_write(&loc_href);
    //remove the hash at the end
    if let Some(pos) = loc_href.find("#") {
        loc_href = loc_href[..pos].to_string();
    }
    logmod::debug_write(&loc_href);
    loc_href.push_str("mem6ws/");

    //send the client ws id as url_param for the first connect
    //and reconnect on lost connection
    loc_href.push_str(client_ws_id.to_string().as_str());
    /*
        logmod::debug_write(&format!(
            "location_href {}  loc_href {} client_ws_id {}",
            location_href, loc_href, client_ws_id
        ));
    */

    //same server address and port as http server
    //for reconnect the old ws id will be an url param
    let ws = unwrap!(WebSocket::new(&loc_href), "WebSocket failed to connect.");

    //I don't know why is clone needed
    let ws_c = ws.clone();
    //It looks that the first send is in some way a handshake and is part of the connection
    //it will be execute on open as a closure
    let open_handler = Box::new(move || {
        //logmod::debug_write("Connection opened, sending MsgRequestWsUid to server");
        unwrap!(
            ws_c.send_with_str(&unwrap!(serde_json::to_string(
                &WsMessage::MsgRequestWsUid {
                    my_ws_uid: client_ws_id,
                    players_ws_uid: players_ws_uid.clone(),
                }
            )),),
            "Failed to send 'test' to server"
        );
        //region heartbeat ping pong keepalive
        let ws2 = ws_c.clone();
        let timeout = gloo_timers::callback::Interval::new(10_000, move || {
            // Do something after the one second timeout is up!
            let usize_time = usize_time();
            let msg = WsMessage::MsgPing { msg_id: usize_time };
            ws_send_msg(ws2.as_ref(), &msg);
            //logmod::console_log(format!("gloo timer: {}", usize_time).as_str());
        });
        // Since we don't plan on cancelling the timeout, call `forget`.
        timeout.forget();
        //endregion
    });

    let cb_oh: Closure<dyn Fn()> = Closure::wrap(open_handler);
    ws.set_onopen(Some(cb_oh.as_ref().unchecked_ref()));

    //don't drop the open_handler memory
    cb_oh.forget();

    ws
}

///usize of time
#[allow(clippy::integer_arithmetic)] //it will not overflow, the minutes are max 60
pub fn usize_time() -> usize {
    let now = js_sys::Date::new_0();
    now.get_minutes() as usize * 100_000 as usize
        + now.get_seconds() as usize * 1_000 as usize
        + now.get_milliseconds() as usize
}

/// receive WebSocket msg callback.
#[allow(clippy::unneeded_field_pattern)]
#[allow(clippy::too_many_lines)] // I know is long
pub fn setup_ws_msg_recv(ws: &WebSocket, vdom: dodrio::VdomWeak) {
    let msg_recv_handler = Box::new(move |msg: JsValue| {
        let data: JsValue = unwrap!(
            Reflect::get(&msg, &"data".into()),
            "No 'data' field in WebSocket message!"
        );
        let data = unwrap!(data.as_string());
        //don't log ping pong there are too much
        if !(data.to_string().contains("MsgPing") || data.to_string().contains("MsgPong")) {
            //logmod::debug_write(&data);
        }
        //serde_json can find out the variant of WsMessage
        //parse json and put data in the enum
        let msg: WsMessage = serde_json::from_str(&data).unwrap_or_else(|_x| WsMessage::MsgDummy {
            dummy: String::from("error"),
        });

        //match enum by variant and prepares the future that will be executed on the next tick
        //in this big enum I put only boilerplate code that don't change any data.
        //for changing data I put code in separate functions for easy reading.
        spawn_local({
            let vdom = vdom.clone();
            async move {
                let _result = vdom
                    .with_component({
                        let vdom = vdom.clone();
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();

                            match msg {
                                //I don't know why I need a dummy, but is entertaining to have one.
                                WsMessage::MsgDummy { dummy } => {
                                    logmod::debug_write(dummy.as_str())
                                }
                                WsMessage::MsgPing { msg_id: _ } => {
                                    unreachable!("mem6 wasm must not receive MsgPing");
                                }
                                WsMessage::MsgPong { msg_id: _ } => {
                                    //logmod::debug_write(format!("MsgPong {}", msg_id).as_str())
                                }
                                WsMessage::MsgRequestWsUid {
                                    my_ws_uid,
                                    players_ws_uid: _,
                                } => logmod::debug_write(
                                    format!("MsgRequestWsUid {}", my_ws_uid).as_str(),
                                ),
                                WsMessage::MsgResponseWsUid {
                                    your_ws_uid,
                                    server_version: _,
                                } => {
                                    //logmod::debug_write(&format!("MsgResponseWsUid: {}  ", your_ws_uid));
                                    on_response_ws_uid(rrc, your_ws_uid);
                                }
                                WsMessage::MsgJoin {
                                    my_ws_uid,
                                    players_ws_uid: _,
                                    my_nickname,
                                } => {
                                    statusjoinedmod::on_msg_joined(rrc, my_ws_uid, my_nickname);
                                    vdom.schedule_render();
                                }
                                WsMessage::MsgStartGame {
                                    my_ws_uid: _,
                                    players_ws_uid: _,
                                    card_grid_data,
                                    game_config,
                                    players,
                                    game_name,
                                } => {
                                    let vdom = vdom.clone();
                                    statusgamedatainitmod::on_msg_start_game(
                                        rrc,
                                        &card_grid_data,
                                        &game_config,
                                        &players,
                                        &game_name,
                                    );
                                    fncallermod::open_new_local_page("#p11");
                                    vdom.schedule_render();
                                }
                                WsMessage::MsgClick1stCard {
                                    my_ws_uid,
                                    players_ws_uid: _,
                                    card_index_of_first_click,
                                    msg_id,
                                } => {
                                    status1stcardmod::on_msg_click_1st_card(
                                        rrc,
                                        &vdom,
                                        my_ws_uid,
                                        card_index_of_first_click,
                                        msg_id,
                                    );
                                    vdom.schedule_render();
                                }
                                WsMessage::MsgClick2ndCard {
                                    my_ws_uid,
                                    players_ws_uid: _,
                                    card_index_of_second_click,
                                    is_point,
                                    msg_id,
                                } => {
                                    status2ndcardmod::on_msg_click_2nd_card(
                                        rrc,
                                        my_ws_uid,
                                        card_index_of_second_click,
                                        is_point,
                                        msg_id,
                                    );
                                    vdom.schedule_render();
                                }
                                WsMessage::MsgDrinkEnd{
                                    my_ws_uid,
                                    players_ws_uid: _,
                                } => {
                                    statusdrinkmod::on_msg_drink_end(rrc, my_ws_uid,&vdom,);
                                    vdom.schedule_render();
                                }
                                WsMessage::MsgTakeTurn {
                                    my_ws_uid,
                                    players_ws_uid: _,
                                    msg_id,
                                } => {
                                    statustaketurnmod::on_msg_take_turn(rrc, my_ws_uid, msg_id);
                                    vdom.schedule_render();
                                }
                                WsMessage::MsgGameOver {
                                    my_ws_uid: _,
                                    players_ws_uid: _,
                                } => {
                                    statusgameovermod::on_msg_game_over(rrc);
                                    vdom.schedule_render();
                                }
                                WsMessage::MsgPlayAgain {
                                    my_ws_uid: _,
                                    players_ws_uid: _,
                                } => {
                                    statusgameovermod::on_msg_play_again(rrc);
                                }
                                
                                WsMessage::MsgAck {
                                    my_ws_uid,
                                    players_ws_uid: _,
                                    msg_id,
                                    msg_ack_kind,
                                } => {
                                    match msg_ack_kind {
                                        MsgAckKind::MsgTakeTurn => {
                                            statustaketurnmod::on_msg_ack_take_turn(
                                                rrc, my_ws_uid, msg_id,
                                            );
                                        }
                                        MsgAckKind::MsgClick1stCard => {
                                            status1stcardmod::on_msg_ack_click_1st_card(
                                                rrc, my_ws_uid, msg_id,
                                            );
                                        }
                                        MsgAckKind::MsgClick2ndCard => {
                                            status2ndcardmod::on_msg_ack_player_click2nd_card(
                                                rrc, my_ws_uid, msg_id,
                                            );
                                        }
                                    }
                                    vdom.schedule_render();
                                }
                                WsMessage::MsgAskPlayer1ForResync {
                                    my_ws_uid: _,
                                    players_ws_uid: _,
                                } => {
                                    websocketreconnectmod::send_msg_for_resync(
                                        rrc,
                                        rrc.game_data.my_ws_uid,
                                    );
                                    vdom.schedule_render();
                                }
                                WsMessage::MsgAllGameData {
                                    my_ws_uid: _,
                                    players_ws_uid: _,
                                    players,
                                    card_grid_data,
                                    card_index_of_first_click,
                                    card_index_of_second_click,
                                    player_turn,
                                    game_status,
                                } => {
                                    websocketreconnectmod::on_msg_all_game_data(
                                        rrc,
                                        players,
                                        card_grid_data,
                                        card_index_of_first_click,
                                        card_index_of_second_click,
                                        player_turn,
                                        game_status,
                                    );
                                    vdom.schedule_render();
                                }
                            }
                        }
                    })
                    .await;
            }
        });
    });

    //magic ??
    let cb_mrh: Closure<dyn Fn(JsValue)> = Closure::wrap(msg_recv_handler);
    ws.set_onmessage(Some(cb_mrh.as_ref().unchecked_ref()));

    //don't drop the event listener from memory
    cb_mrh.forget();
}

/// on error write it on the screen for debugging
pub fn setup_ws_onerror(ws: &WebSocket, vdom: dodrio::VdomWeak) {
    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        let err_text = format!("error event {:?}", e);
        //logmod::debug_write(&err_text);
        {
            spawn_local({
                let vdom = vdom.clone();
                async move {
                    let _result = vdom
                        .with_component({
                            let vdom = vdom.clone();
                            move |root| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                rrc.game_data.error_text = err_text;
                                vdom.schedule_render();
                            }
                        })
                        .await;
                }
            });
        }
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();
}
/// on close WebSocket connection
pub fn setup_ws_onclose(ws: &WebSocket, vdom: dodrio::VdomWeak) {
    let onclose_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        let err_text = format!("ws_onclose {:?}", e);
        logmod::debug_write(&format!("onclose_callback {}", &err_text));
        {
            spawn_local({
                let vdom = vdom.clone();
                async move {
                    let _result = vdom
                        .with_component({
                            let vdom = vdom.clone();
                            move |root| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                //I want to show a reconnect button to the user
                                rrc.game_data.is_reconnect = true;
                                vdom.schedule_render();
                            }
                        })
                        .await;
                }
            });
        }
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
}
///setup all ws events
pub fn setup_all_ws_events(ws: &WebSocket, vdom: dodrio::VdomWeak) {
    //WebSocket on receive message callback
    setup_ws_msg_recv(ws, vdom.clone());

    //WebSocket on error message callback
    setup_ws_onerror(ws, vdom.clone());

    //WebSocket on close message callback
    setup_ws_onclose(ws, vdom);
}

///generic send ws message
pub fn ws_send_msg(ws: &WebSocket, ws_message: &WsMessage) {
    let x = ws.send_with_str(&unwrap!(serde_json::to_string(ws_message)));
    // retry send a 5 times before panicking
    if let Err(err) = x {
        let ws = ws.clone();
        let ws_message = ws_message.clone();
        spawn_local({
            let ws = ws.clone();
            let ws_message = ws_message.clone();
            async move {
                let mut retries: usize = 1;
                while retries <= 5 {
                    logmod::debug_write(&format!("send retries: {}", retries));
                    //Wait 10 ms
                    TimeoutFuture::new(10).await;
                    let x = ws.send_with_str(&unwrap!(serde_json::to_string(&ws_message)));
                    if let Ok(y) = x {
                        break;
                    }
                    retries += 1;
                }
                if retries == 0 {
                    panic!("error 5 times retry ws_send_msg");
                }
            }
        });
    }
}

///msg response with ws_uid, just to check.
pub fn on_response_ws_uid(rrc: &mut RootRenderingComponent, your_ws_uid: usize) {
    if rrc.game_data.my_ws_uid != your_ws_uid {
        rrc.game_data.error_text = "my_ws_uid is incorrect!".to_string();
    }
}
