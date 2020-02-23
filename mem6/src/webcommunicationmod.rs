// webcommunicationmod.rs
//! structs and methods around web and communication

//region: use
//use crate::*;
use mem6_common::*;

use web_sys::WebSocket;
use serde_derive::{Serialize, Deserialize};
//use unwrap::unwrap;
//endregion

//region: structs
/// save the message in queue to resend it if timeout expires
#[derive(Serialize, Deserialize)]
pub struct MsgInQueue {
    /// the player that must ack the msg
    pub player_ws_uid: usize,
    /// the msg id is a random number
    pub msg_id: usize,
    /// the content of the message if it needs to be resend
    pub msg: WsMessage,
}

/// game data
pub struct WebCommunication {
    /// web socket. used it to send message onclick.
    pub ws: WebSocket,
    /// local # hash route
    pub local_route: String,
    /// downloaded html template
    pub html_template: String,
    /// is reconnect
    pub is_reconnect: bool,
    /// my ws client instance unique id. To not listen the echo to yourself.
    pub my_ws_uid: usize,
    /// the json string for the ws server to send msgs to other players only
    pub msg_receivers: String,
    /// error text
    pub error_text: String,
    /// href
    pub href: String,
    /// href hash the local page #
    pub href_hash: String,
    /// vector of msgs waiting for ack. If the 3 sec timeout passes it resends the same msg.
    pub msgs_waiting_ack: Vec<MsgInQueue>,
    /// show debug info on the smartphone screen
    pub show_debug_info: bool,
}
//endregion

impl WebCommunication {
    /// constructor
    pub fn new(ws: WebSocket, my_ws_uid: usize, msg_receivers: String) -> Self {
        // return from constructor
        WebCommunication {
            ws,
            local_route: "".to_owned(),
            html_template: "".to_owned(),
            is_reconnect: false,
            my_ws_uid,
            msg_receivers,
            error_text: "".to_string(),
            href: "".to_string(),
            href_hash: "".to_string(),
            msgs_waiting_ack: vec![],
            show_debug_info: false,
        }
    }
}
