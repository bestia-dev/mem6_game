//! fncallermod  

use crate::logmod;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::divnicknamemod;

use wasm_bindgen::JsValue;
use unwrap::unwrap;

/// html_templating functions that return a String
pub fn call_function_string(rrc: &RootRenderingComponent, sx: &str) -> String {
    logmod::debug_write(&format!("call_function_string: {}", &sx));
    match sx {
        "my_nickname" => rrc.game_data.my_nickname.to_owned(),
        "blink_or_not" => divnicknamemod::blink_or_not(rrc),
        _ => {
            let x = format!("Error: Unrecognized call_function_string: {}", sx);
            logmod::debug_write(&x);
            x
        }
    }
}
/// html_templating functions for listeners
/// get a clone of the VdomWeak
pub fn call_listener(vdom: &dodrio::VdomWeak, rrc: &RootRenderingComponent, sx: String) {
    logmod::debug_write(&format!("call_listener: {}", &sx));
    match sx.as_str() {
        "nickname_onkeyup" => {
            divnicknamemod::nickname_onkeyup(vdom);
        }
        "start_a_group_onclick" => {
            logmod::debug_write("start a group on click");
            // an example how to change the local_route from code
            let window = unwrap!(web_sys::window());
            let _x = window.location().set_hash("#02");
        }
        "join_a_group_onclick" => {
            logmod::debug_write("join a group on click");
            // an example how to change the local_route from code
            let window = unwrap!(web_sys::window());
            let _x = window.location().set_hash("#03");
        }
        _ => {
            let x = format!("Error: Unrecognized call_listener: {}", sx);
            logmod::debug_write(&x);
        }
    }
}

//TODO: html_templating functions that return a Node

/// get local route hash #
pub fn get_local_route(rrc: &RootRenderingComponent) -> String {
    let gt = format!("get local_route from rrc: {}", &rrc.local_route);
    gt
}

/// get red color
pub fn get_red() -> String {
    "red".to_owned()
}
