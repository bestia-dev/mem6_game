// websocketboilermod.rs
//! module that cares about WebSocket communication

#![allow(clippy::panic)]

// region: use
use rust_wasm_websys_utils::*;

use unwrap::unwrap;
use js_sys::Reflect;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{ErrorEvent, WebSocket};
use gloo_timers::future::TimeoutFuture;
use serde_derive::{Serialize, Deserialize};
use dodrio::{VdomWeak,RootRender};
// endregion

/// WsMessageToServer enum for WebSocket
/// The ws server will perform an action according to this type.
#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessageToServer {
    /// Request WebSocket Uid - first message to WebSocket server
    MsgRequestWsUid {
        /// ws client instance unique id. To not listen the echo to yourself.
        msg_sender_ws_uid: usize,
    },
    /// MsgPing
    MsgPing {
        /// random msg_id
        msg_id: u32,
    },
}

/// WsMessageFromServer enum for WebSocket
/// The ws server will send this kind of msgs.
#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessageFromServer {
    /// response from WebSocket server for first message
    MsgResponseWsUid {
        /// WebSocket Uid
        msg_receiver_ws_uid: usize,
        /// server version
        server_version: String,
    },
    /// MsgPong
    MsgPong {
        /// random msg_id
        msg_id: u32,
    },
}

pub trait WebSocketTrait {
    // region: getter setter
    fn get_ws_clone(&self)->WebSocket;
    fn set_ws(&mut self,ws:WebSocket);
    // endregion getter setter
    fn on_response_ws_uid(root: &mut dyn RootRender, msg_receiver_ws_uid: usize) ;
    fn on_msg_recv_for_ws_message_for_receivers( vdom: VdomWeak,data:String) ;
    fn update_on_error(root:&mut dyn RootRender,err_text:String);
    fn update_on_close(root:&mut dyn RootRender);
    // the location_href is not consumed in this function and Clippy wants a reference instead a value
    // but I don't want references, because they have the lifetime problem.
    #[allow(clippy::needless_pass_by_value)]
    /// setup WebSocket connection
    fn setup_ws_connection(&mut self, location_href: String, client_ws_id: usize) -> WebSocket {
        // web-sys has WebSocket for Rust exactly like JavaScript has¸
        // location_href comes in this format  http:// localhost:4000/
        let mut loc_href = location_href
            .replace("http://", "ws://")
            .replace("https://", "wss://");
        // Only for debugging in the development environment
        // let mut loc_href = String::from("ws://192.168.1.57:80/");
        websysmod::debug_write(&loc_href);
        // remove the hash at the end
        if let Some(pos) = loc_href.find('#') {
            loc_href = unwrap!(loc_href.get(..pos)).to_string();
        }
        websysmod::debug_write(&loc_href);
        loc_href.push_str("mem6ws/");

        // send the client ws id as url_param for the first connect
        // and reconnect on lost connection
        loc_href.push_str(client_ws_id.to_string().as_str());
        /*
            websysmod::debug_write(&format!(
                "location_href {}  loc_href {} client_ws_id {}",
                location_href, loc_href, client_ws_id
            ));
        */

        // same server address and port as http server
        // for reconnect the old ws id will be an url param
        let ws = unwrap!(WebSocket::new(&loc_href), "WebSocket failed to connect.");
        self.set_ws(ws);
        let ws0 = self.get_ws_clone();
        // I don't know why is clone needed
        // It looks that the first send is in some way a handshake and is part of the connection
        // it will be execute on open as a closure
        let ws1 = self.get_ws_clone();
        let open_handler = Box::new(move || {
            // websysmod::debug_write("Connection opened, sending MsgRequestWsUid to server");
            Self::send_to_server_msg_request_ws_uid(ws1.clone(),client_ws_id);
            // region heartbeat ping pong keepalive
            let ws2 = ws1.clone();
            let timer_interval = gloo_timers::callback::Interval::new(10_000, move || {
                // Do something after the one second timer_interval is up!
                Self::send_to_server_msg_ping(ws2.clone(),time_now_in_minutes());
                // websysmod::console_log(format!("gloo timer: {}", time_now_in_minutes).as_str());
            });
            // Since we don't plan on cancelling the timer_interval, call `forget`.
            timer_interval.forget();
            // endregion
        });

        let cb_oh: Closure<dyn Fn()> = Closure::wrap(open_handler);
        ws0.set_onopen(Some(cb_oh.as_ref().unchecked_ref()));

        // don't drop the open_handler memory
        cb_oh.forget();

        ws0
    }
    /// setup all ws events
    fn setup_all_ws_events(ws: &WebSocket, vdom: VdomWeak) {
        // WebSocket on receive message callback
        Self::setup_ws_msg_recv(ws, vdom.clone());

        // WebSocket on error message callback
        Self::setup_ws_onerror(ws, vdom.clone());

        // WebSocket on close message callback
        Self::setup_ws_onclose(ws, vdom.clone());
    }
    /// receive WebSocket msg callback.
    #[allow(clippy::unneeded_field_pattern)]
    #[allow(clippy::too_many_lines)] // I know is long
    fn setup_ws_msg_recv(ws: &WebSocket, vdom: VdomWeak) {
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
                                            Self::on_response_ws_uid(root, msg_receiver_ws_uid);
                                        }
                                    }
                                }
                            })
                            .await;
                    }
                });
            } else {
                
                Self::on_msg_recv_for_ws_message_for_receivers( vdom.clone(), data);
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
    fn setup_ws_onerror(ws: &WebSocket, vdom: VdomWeak) {
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
                                    Self::update_on_error(root,err_text);
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
    fn setup_ws_onclose(ws: &WebSocket, vdom: VdomWeak) {
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
                                    Self::update_on_close(root);
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

    fn send_to_server_msg_ping(ws:WebSocket ,msg_id:u32){
        let ws_message = WsMessageToServer::MsgPing { msg_id };
        let json_message = unwrap!(serde_json::to_string(&ws_message));
        Self::ws_send_msg_with_retry(&ws, json_message);
    }
    
    fn send_to_server_msg_request_ws_uid(ws:WebSocket,client_ws_id:usize ){
        unwrap!(ws.send_with_str(&unwrap!(serde_json::to_string(
            &WsMessageToServer::MsgRequestWsUid {
                msg_sender_ws_uid: client_ws_id
            }
        ))));
    }
    
    /// send ws message to other players
    fn ws_send_msg_with_retry(ws: &WebSocket, json_message:String) {
        let x = ws.send_with_str(&json_message);
        // retry send a 10 times before panicking
        if let Err(_err) = x {
            let ws = ws.clone();
            spawn_local({
                async move {
                    let mut retries: usize = 1;
                    while retries <= 10 {
                        websysmod::debug_write(&format!("send retries: {}", retries));
                        // Wait 100 ms
                        TimeoutFuture::new(100).await;
                        let x = ws.send_with_str(&json_message);
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
                        panic!("error 10 times retry ws_send_msg_with_retry");
                    }
                }
            });
        }
    }
}

/// usize of time
#[allow(clippy::integer_arithmetic)]
// usize will not overflow, the minutes are max 60, so 6 mio
pub fn time_now_in_minutes() -> u32 {
    let now = js_sys::Date::new_0();
    now.get_minutes() * 100_000 + now.get_seconds() * 1_000 + now.get_milliseconds()
}
