// websocketimplmod.rs
//! module that cares about WebSocket communication

#![allow(clippy::panic)]

// region: use
use rust_wasm_websys_utils::*;

use unwrap::unwrap;
//use js_sys::Reflect;
use wasm_bindgen::{prelude::*, JsCast};
//use wasm_bindgen_futures::spawn_local;
//use web_sys::{ErrorEvent, WebSocket};
use web_sys::{ WebSocket};
//use gloo_timers::future::TimeoutFuture;
use serde_derive::{Serialize, Deserialize};
//use dodrio::VdomWeak;
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
    //fn get_rtc_ws(&self)->&WebSocket;   
    // endregion getter setter
    fn send_to_server_msg_ping(ws:WebSocket  ,msg_id:u32);
    fn send_to_server_msg_request_ws_uid(ws:WebSocket,client_ws_id:usize );
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
}

/// usize of time
#[allow(clippy::integer_arithmetic)]
// usize will not overflow, the minutes are max 60, so 6 mio
pub fn time_now_in_minutes() -> u32 {
    let now = js_sys::Date::new_0();
    now.get_minutes() * 100_000 + now.get_seconds() * 1_000 + now.get_milliseconds()
}
