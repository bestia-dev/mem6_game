//! fncallermod  

use crate::logmod;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::divnicknamemod;

/// html_templating functions that return a String
pub fn call_function_string(rrc: &RootRenderingComponent, sx: &str) -> String {
    logmod::debug_write(&format!("call_function_string: {}", &sx));
    match sx {
        "my_nickname" => rrc.game_data.my_nickname.to_owned(),
        "blink_or_not" => divnicknamemod::blink_or_not(rrc),
        "first_text" => "this is first text replaced".to_owned(),
        "first_attr" => "this is first attr replaces".to_owned(),
        "get_local_route" => get_local_route(rrc),
        "get_red" => get_red(),
        "local_route" => rrc.local_route.to_owned(),
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
