// webdatamod.rs
//! structs and methods around web and communication

// region: use
use crate::*;
use rust_wasm_websocket::websocketmod::WebSocketTrait;

use serde_derive::{Serialize, Deserialize};
use unwrap::unwrap;
use dodrio::VdomWeak;
// endregion

// region: structs
/// save the message in queue to resend it if timeout expires
#[derive(Serialize, Deserialize)]
pub struct MsgInQueue {
    /// the player that must ack the msg
    pub player_ws_uid: usize,
    /// the msg id is a random number
    pub msg_id: usize,
    /// the content of the message if it needs to be resend
    pub msg: websocketboilermod::WsMessageForReceivers,
}

/// game data
pub struct WebData {
    /// websocket data
    pub websocket_data: websocketboilermod::WebSocketData,
    /// data for web rtc communication
    pub web_rtc_data: webrtcimplmod::WebRtcData,
    /// downloaded html template for main page
    pub html_template: String,
    /// vector of named sub_templates <template name=xxx>...</template>
    pub html_sub_templates: Vec<(String, String)>,
    /// is reconnect
    pub is_reconnect: bool,
    /// my ws client instance unique id. To not listen the echo to yourself.
    pub my_ws_uid: usize,
    /// the json string for the ws server to send msgs to other players only
    pub msg_receivers_json: String,
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
// endregion

impl WebData {
    /// constructor
    pub fn new(my_ws_uid: usize, msg_receivers_json: String) -> Self {
        let websocket_data = websocketboilermod::WebSocketData::new();
        let web_rtc_data = webrtcimplmod::WebRtcData::new(my_ws_uid);
        // return from constructor
        WebData {
            websocket_data,
            web_rtc_data,
            html_template: "".to_owned(),
            html_sub_templates: vec![],
            is_reconnect: false,
            my_ws_uid,
            msg_receivers_json,
            error_text: "".to_string(),
            href: "".to_string(),
            href_hash: "".to_string(),
            msgs_waiting_ack: vec![],
            show_debug_info: false,
        }
    }

    /// get sub_template
    pub fn get_sub_template(&self, template_name: &str) -> String {
        let mut html_template = format!("Error: no sub-template with name: {}", template_name);
        for (name, template) in &self.html_sub_templates {
            if name == template_name {
                html_template = template.to_string();
                break;
            }
        }
        // return
        html_template
    }

    /// create websocket connection
    pub fn start_websocket(&mut self, vdom: VdomWeak) {
        let (location_href, _href_hash) = websysmod::get_url_and_hash();
        //let websocket_data = websocketboilermod::WebSocketData::new();
        let ws = self.websocket_data.setup_ws_connection(location_href.clone(), self.my_ws_uid);
        websocketboilermod::WebSocketData::setup_all_ws_events(&ws, vdom);
    }
    /// send msg over ws
    pub fn send_ws_msg_from_web_data(&self, ws_message: &websocketboilermod::WsMessageForReceivers) {
        let json_message = unwrap!(serde_json::to_string(ws_message));
        websocketboilermod::WebSocketData::ws_send_msg_with_retry(unwrap!(self.websocket_data.ws.as_ref()), json_message);
    }
}
    