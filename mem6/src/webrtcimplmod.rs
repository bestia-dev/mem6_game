// webrtcimplmod.rs
//! specific implementation of WebRTC communication

// region: use
use crate::*;
//for trait rrc.render_template
use crate::htmltemplatemod::HtmlTemplating;
use crate::webrtcmod::{WebRtcData};

use unwrap::unwrap;
use wasm_bindgen::{JsCast};
//use wasm_bindgen_futures::spawn_local;
use dodrio::{RenderContext, Node, VdomWeak, RootRender};
//use serde_derive::{Serialize, Deserialize};
// endregion

impl webrtcmod::WebRtcTrait for WebRtcData {
    fn get_web_rtc_data_from_root_render(root: &mut dyn RootRender) -> &mut WebRtcData {
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        //return
        &mut rrc.web_rtc_data
    }
}
/// on key up only for Enter
pub fn web_rtc_receiver_ws_uid_onkeyup(
    vdom: VdomWeak,
    rrc: &mut RootRenderingComponent,
    event: web_sys::Event,
) {
    let keyboard_event = event.dyn_into::<web_sys::KeyboardEvent>();
    //websysmod::debug_write(&format!("web_rtc_receiver_ws_uid_onkeyup: {:?}",&keyboard_event));
    if let Ok(keyboard_event) = keyboard_event {
        //websysmod::debug_write(&keyboard_event.key());
        if keyboard_event.key() == "Enter" {
            // same as button click
            rrc.web_rtc_data
                .web_rtc_start(vdom, unwrap!(rrc.web_data.ws.clone()));
        }
    }
}

/// on key up only for Enter
pub fn web_rtc_chat_text_onkeyup(
    vdom: VdomWeak,
    rrc: &mut RootRenderingComponent,
    event: web_sys::Event,
) {
    let keyboard_event = event.dyn_into::<web_sys::KeyboardEvent>();
    //websysmod::debug_write(&format!("on key up: {:?}",&keyboard_event));
    if let Ok(keyboard_event) = keyboard_event {
        // websysmod::debug_write(&keyboard_event.key());
        if keyboard_event.key() == "Enter" {
            // same as button click
            rrc.web_rtc_data.web_rtc_send_chat(vdom);
        } else {
            rrc.web_rtc_data.rtc_my_message =
                websysmod::get_input_element_value_string_by_id("web_rtc_chat_text");
        }
    }
}

/// render messages
pub fn web_rtc_div_messages<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
) -> Vec<Node<'a>> {
    let mut vec_nodes = Vec::<Node>::new();

    let mut index = rrc.web_rtc_data.rtc_chat.len();
    // reverse a vector old school
    if index > 0 {
        index -= 1;
        loop {
            let chat_msg = &rrc.web_rtc_data.rtc_chat[index];
            let template_name = format!("message_sender{}", chat_msg.sender);
            let mut html_template = rrc.web_data.get_sub_template(&template_name);
            html_template = html_template.replace("replace_in_code_with_msg", &chat_msg.msg);
            let node =
                unwrap!(rrc.render_template(cx, &html_template, htmltemplatemod::HtmlOrSvg::Html));
            vec_nodes.push(node);
            if index == 0 {
                break;
            } else {
                index -= 1;
            }
        }
    }
    //return
    vec_nodes
}
