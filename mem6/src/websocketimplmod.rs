// websocketimplmod.rs
//! module that cares about WebSocket communication

#![allow(clippy::panic)]

// region: use
use crate::*;
use rust_wasm_websocket::websocketmod::WebSocketTrait;
use rust_wasm_webrtc::webrtcmod::WebRtcTrait;
use mem6_common::*;

use unwrap::unwrap;
use js_sys::Reflect;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{ErrorEvent, WebSocket};
use gloo_timers::future::TimeoutFuture;
use serde_derive::{Serialize, Deserialize};
use dodrio::VdomWeak;
// endregion


/// message for receivers
/// The server has a copy of this declaration, but without the msg_data
#[derive(Serialize, Deserialize, Clone)]
pub struct WsMessageForReceivers {
    /// ws client instance unique id. To not listen the echo to yourself.
    pub msg_sender_ws_uid: usize,
    /// only the players that reconnected
    pub msg_receivers_json: String,
    /// msg data
    pub msg_data: WsMessageGameData,
}

pub struct WebSocketData{
    /// web socket communication between players
    pub ws: Option<WebSocket>,
}

impl WebSocketData {
    /// constructor
    pub fn new() -> Self {
        // return from constructor
        Self {ws:None}
    }
}

impl WebSocketTrait for WebSocketData {
    
    fn send_to_server_msg_ping(ws:WebSocket ,msg_id:u32){
        let msg = WsMessageToServer::MsgPing { msg_id };
        Self::ws_send_msg_to_server(&ws, &msg);
    }
    fn send_to_server_msg_request_ws_uid(ws:WebSocket,client_ws_id:usize ){
        unwrap!(ws.send_with_str(&unwrap!(serde_json::to_string(
            &WsMessageToServer::MsgRequestWsUid {
                msg_sender_ws_uid: client_ws_id
            }
        ))));
    }
    /// send ws message to server
    fn ws_send_msg_to_server(ws: &WebSocket, ws_message: &WsMessageToServer) {
        let x = ws.send_with_str(&unwrap!(serde_json::to_string(ws_message)));
        // retry send a 10 times before panicking
        if let Err(_err) = x {
            let ws = ws.clone();
            let ws_message = ws_message.clone();
            spawn_local({
                async move {
                    let mut retries: usize = 1;
                    while retries <= 10 {
                        websysmod::debug_write(&format!("send retries: {}", retries));
                        // Wait 100 ms
                        TimeoutFuture::new(100).await;
                        let x = ws.send_with_str(&unwrap!(serde_json::to_string(&ws_message)));
                        if let Ok(_y) = x {
                            break;
                        }
                        // this will go until 10 and cannot overflow
                        #[allow(clippy::integer_arithmetic)]
                        {
                            retries += 1;
                        }
                    }
                    if retries == 0 {
                        panic!("error 10 times retry ws_send_msg");
                    }
                }
            });
        }
    }

}

/// receive WebSocket msg callback.
#[allow(clippy::unneeded_field_pattern)]
#[allow(clippy::too_many_lines)] // I know is long
pub fn setup_ws_msg_recv(ws: &WebSocket, vdom: VdomWeak) {
    let msg_recv_handler = Box::new(move |msg: JsValue| {
        let data: JsValue = unwrap!(Reflect::get(&msg, &"data".into()));
        let data = unwrap!(data.as_string());

        // don't log ping pong there are too much
        //if !data.to_string().contains("MsgPong") {
        // websysmod::debug_write(&data);
        //}

        // we can receive 2 types of msgs:
        // 1. from the server WsMessageFromServer
        // 2. from other players WsMessage
        if let Ok(msg) = serde_json::from_str::<WsMessageFromServer>(&data) {
            //msg from ws server
            spawn_local({
                let vdom_on_next_tick = vdom.clone();
                async move {
                    let _result = vdom_on_next_tick
                        .with_component({
                            //let vdom = vdom_on_next_tick.clone();
                            move |root| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                // msgs from server
                                match msg {
                                    WsMessageFromServer::MsgPong { msg_id: _ } => {
                                        // websysmod::debug_write(format!("MsgPong {}", msg_id).as_str())
                                    }
                                    WsMessageFromServer::MsgResponseWsUid {
                                        msg_receiver_ws_uid,
                                        server_version: _,
                                    } => {
                                        // websysmod::debug_write(&format!("MsgResponseWsUid: {}  ", msg_receiver_ws_uid));
                                        on_response_ws_uid(rrc, msg_receiver_ws_uid);
                                    }
                                }
                            }
                        })
                        .await;
                }
            });
        } else {
            // msg from ws clients (players)
            // serde_json can find out the variant of WsMessage
            // parse json and put data in the enum
            if let Ok(msg) = serde_json::from_str::<WsMessageForReceivers>(&data) {
                // match enum by variant and prepares the future that will be executed on the next tick
                // in this big enum I put only boilerplate code that don't change any data.
                // for changing data I put code in separate functions for easy reading.
                spawn_local({
                    let vdom_on_next_tick = vdom.clone();
                    async move {
                        let _result = vdom_on_next_tick
                            .with_component({
                                let vdom = vdom_on_next_tick.clone();
                                move |root| {
                                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                    match msg.msg_data {
                                        WsMessageGameData::MsgJoin {
                                            my_nickname,
                                        } => {
                                            statusjoinedmod::on_msg_joined(rrc, msg.msg_sender_ws_uid, my_nickname);
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgStartGame {
                                            
                                            card_grid_data,
                                            game_config,
                                            players,
                                            game_name,
                                            player_turn,
                                        } => {
                                            statusgamedatainitmod::on_msg_start_game(
                                                rrc,
                                                &card_grid_data,
                                                &game_config,
                                                &players,
                                                &game_name,
                                                player_turn,
                                            );
                                            htmltemplateimplmod::open_new_local_page("#p11");
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgClick1stCard {
                                            
                                            card_index_of_1st_click,
                                            msg_id,
                                        } => {
                                            status1stcardmod::on_msg_click_1st_card(
                                                rrc,
                                                vdom.clone(),
                                                msg.msg_sender_ws_uid,
                                                card_index_of_1st_click,
                                                msg_id,
                                            );
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgClick2ndCard {
                                            card_index_of_2nd_click,
                                            is_point,
                                            msg_id,
                                        } => {
                                            status2ndcardmod::on_msg_click_2nd_card(
                                                rrc,
                                                msg.msg_sender_ws_uid,
                                                card_index_of_2nd_click,
                                                is_point,
                                                msg_id,
                                            );
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgDrinkEnd {
                                           
                                        } => {
                                            statusdrinkmod::on_msg_drink_end(rrc, msg.msg_sender_ws_uid, vdom.clone());
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgTakeTurn {
                                            
                                            
                                            msg_id,
                                        } => {
                                            statustaketurnmod::on_msg_take_turn(rrc, msg.msg_sender_ws_uid, msg_id);
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgGameOver {
                                           
                                        } => {
                                            statusgameovermod::on_msg_game_over(rrc);
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgPlayAgain {
                                                                                   } => {
                                            statusgameovermod::on_msg_play_again(rrc);
                                        }
                                        WsMessageGameData::MsgSoundsAndLabels {
                                            sounds_and_labels,
                                        
                                           
                                        } => {
                                            rrc.game_data.sounds_and_labels=sounds_and_labels;
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgAck {
                                           
                                            msg_id,
                                            msg_ack_kind,
                                        } => {
                                            match msg_ack_kind {
                                                MsgAckKind::MsgTakeTurn => {
                                                    statustaketurnmod::on_msg_ack_take_turn(
                                                        rrc, msg.msg_sender_ws_uid, msg_id,
                                                    );
                                                }
                                                MsgAckKind::MsgClick1stCard => {
                                                    status1stcardmod::on_msg_ack_click_1st_card(
                                                        rrc, msg.msg_sender_ws_uid, msg_id,
                                                    );
                                                }
                                                MsgAckKind::MsgClick2ndCard => {
                                                    status2ndcardmod::on_msg_ack_player_click2nd_card(
                                                        rrc, msg.msg_sender_ws_uid, msg_id, vdom.clone(),
                                                    );
                                                }
                                            }
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgAskPlayer1ForResync {
                                            
                                        } => {
                                            statusreconnectmod::send_msg_for_resync(rrc);
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgAllGameData {
                                            
                                            players,
                                            card_grid_data,
                                            card_index_of_1st_click,
                                            card_index_of_2nd_click,
                                            player_turn,
                                            game_status,
                                        } => {
                                            statusreconnectmod::on_msg_all_game_data(
                                                rrc,
                                                players,
                                                card_grid_data,
                                                card_index_of_1st_click,
                                                card_index_of_2nd_click,
                                                player_turn,
                                                game_status,
                                            );
                                            vdom.schedule_render();
                                        }
                                        WsMessageGameData::MsgWebrtcOffer{
                                            sdp
                                        }=>{
                                            rrc.web_data.web_rtc_data.web_rtc_receive_offer(vdom.clone(),sdp, msg.msg_sender_ws_uid);
                                        }
                                        WsMessageGameData::MsgWebrtcAnswer{
                                            sdp
                                        }=>{
                                            rrc.web_data.web_rtc_data.web_rtc_receive_answer(vdom.clone(),sdp);
                                        }
                                        WsMessageGameData::MsgWebrtcIceCandidate{
                                            sdp
                                        }=>{
                                            rrc.web_data.web_rtc_data.web_rtc_receive_ice_candidate(vdom.clone(),sdp);
                                        }
                                    }
                                }
                            })
                            .await;
                    }
                });
            } else {
                //unknown message
            }
        }
    });

    // magic ??
    let cb_mrh: Closure<dyn Fn(JsValue)> = Closure::wrap(msg_recv_handler);
    ws.set_onmessage(Some(cb_mrh.as_ref().unchecked_ref()));

    // don't drop the event listener from memory
    cb_mrh.forget();
}

/// on error write it on the screen for debugging
#[allow(clippy::as_conversions)]
pub fn setup_ws_onerror(ws: &WebSocket, vdom: VdomWeak) {
    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        let err_text = format!("error event {:?}", e);
        // websysmod::debug_write(&err_text);
        {
            spawn_local({
                let vdom_on_next_tick = vdom.clone();
                async move {
                    let _result = vdom_on_next_tick
                        .with_component({
                            let vdom = vdom_on_next_tick.clone();
                            move |root| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                rrc.web_data.error_text = err_text;
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
#[allow(clippy::as_conversions)]
pub fn setup_ws_onclose(ws: &WebSocket, vdom: VdomWeak) {
    let onclose_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        let err_text = format!("ws_onclose {:?}", e);
        websysmod::debug_write(&format!("onclose_callback {}", &err_text));
        {
            spawn_local({
                let vdom_on_next_tick = vdom.clone();
                async move {
                    let _result = vdom_on_next_tick
                        .with_component({
                            let vdom = vdom_on_next_tick.clone();
                            move |root| {
                                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                                // I want to show a reconnect button to the user
                                rrc.web_data.is_reconnect = true;
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
/// setup all ws events
pub fn setup_all_ws_events(ws: &WebSocket, vdom: VdomWeak) {
    // WebSocket on receive message callback
    setup_ws_msg_recv(ws, vdom.clone());

    // WebSocket on error message callback
    setup_ws_onerror(ws, vdom.clone());

    // WebSocket on close message callback
    setup_ws_onclose(ws, vdom.clone());
}


/// send ws message to other players
pub fn ws_send_msg(ws: &WebSocket, ws_message: &WsMessageForReceivers) {
    let x = ws.send_with_str(&unwrap!(serde_json::to_string(ws_message)));
    // retry send a 10 times before panicking
    if let Err(_err) = x {
        let ws = ws.clone();
        let ws_message = ws_message.clone();
        spawn_local({
            async move {
                let mut retries: usize = 1;
                while retries <= 10 {
                    websysmod::debug_write(&format!("send retries: {}", retries));
                    // Wait 100 ms
                    TimeoutFuture::new(100).await;
                    let x = ws.send_with_str(&unwrap!(serde_json::to_string(&ws_message)));
                    if let Ok(_y) = x {
                        break;
                    }
                    // this will go until 10 and cannot overflow
                    #[allow(clippy::integer_arithmetic)]
                    {
                        retries += 1;
                    }
                }
                if retries == 0 {
                    panic!("error 10 times retry ws_send_msg");
                }
            }
        });
    }
}

/// msg response with ws_uid, just to check.
pub fn on_response_ws_uid(rrc: &mut RootRenderingComponent, msg_receiver_ws_uid: usize) {
    if rrc.web_data.my_ws_uid != msg_receiver_ws_uid {
        rrc.web_data.error_text = "my_ws_uid is incorrect!".to_string();
    }
}

/// my_ws_uid is random generated on the client and sent to
/// WebSocket server with an url param.
/// It is saved locally to allow reconnection
/// if there are connection problems.
pub fn load_or_random_ws_uid() -> usize {
    let mut my_ws_uid: usize = storagemod::load_my_ws_uid();
    if my_ws_uid == 0 {
        my_ws_uid = websysmod::get_random(1, 9999);
        storagemod::save_my_ws_uid(my_ws_uid);
    }
    // return
    my_ws_uid
}
