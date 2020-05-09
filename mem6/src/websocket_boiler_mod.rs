// websocket_boiler_mod.rs
//! Boilerplate code that is part of the library `rust_wasm_websocket`,  
//! but sadly it cannot be encapsulated in the external crate.  
//! This mod should not be modified by the project author.  
//! The specific code for the project is in the file `websocket_spec_mod`.  

#![allow(clippy::panic)]

// region: use
use crate::*;
use rust_wasm_websocket::websocketmod::{WebSocketTrait};

use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;
use web_sys::{ WebSocket};
use serde_derive::{Serialize, Deserialize};
use dodrio::{VdomWeak,RootRender};
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

#[derive( Clone)]
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
    // region: getter setter
    fn get_ws_clone(&self)->WebSocket{
        unwrap!(self.ws.clone())
    }
    fn set_ws(&mut self,ws:WebSocket){
        self.ws=Some(ws);
    }
    // endregion getter setter
    /// msg response with ws_uid, just to check.
    fn on_response_ws_uid(root: &mut dyn RootRender, msg_receiver_ws_uid: usize) {
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        if rrc.web_data.my_ws_uid != msg_receiver_ws_uid {
            rrc.web_data.error_text = "my_ws_uid is incorrect!".to_string();
        }
    }
    fn on_msg_recv_for_ws_message_for_receivers( vdom: VdomWeak,data:String) {
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
                                websocket_spec_mod::match_msg_and_call_function(vdom,rrc,msg);
                            }
                        })
                        .await;
                }
            });
        } else {
            //unknown message
        }
    }
    fn update_on_error(root:&mut dyn RootRender,err_text:String){
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        rrc.web_data.error_text = err_text;
    }
    fn update_on_close(root:&mut dyn RootRender){
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        // I want to show a reconnect button to the user
        rrc.web_data.is_reconnect = true;
    }
}

/// my_ws_uid is random generated on the client and sent to
/// WebSocket server with an url param.
/// It is saved locally to allow reconnection
/// if there are connection problems.
pub fn load_or_random_ws_uid() -> usize {
    let mut my_ws_uid: usize = storage_mod::load_my_ws_uid();
    if my_ws_uid == 0 {
        my_ws_uid = websysmod::get_random(1, 9999);
        storage_mod::save_my_ws_uid(my_ws_uid);
    }
    // return
    my_ws_uid
}
