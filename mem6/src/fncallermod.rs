//! fncallermod  

use crate::logmod;
use crate::rootrenderingcomponentmod::RootRenderingComponent;

/// html_templating functions that return a String
pub fn call_function_string(rrc: &RootRenderingComponent, sx: &str) -> String {
    logmod::debug_write(&format!("call_function_string: {}", &sx));
    match sx {
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

//TODO: html_templating functions that return a Node

pub fn get_local_route(rrc: &RootRenderingComponent) -> String {
    let gt = format!("get local_route from rrc: {}", &rrc.local_route);
    gt
}

pub fn get_red() -> String {
    "red".to_owned()
}
