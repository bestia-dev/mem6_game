//! fncallermod  

use crate::logmod;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::divnicknamemod;
use crate::statusjoinedmod;

use unwrap::unwrap;
use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.

/// html_templating functions that return a String
pub fn call_function_string(rrc: &RootRenderingComponent, sx: &str) -> String {
    //logmod::debug_write(&format!("call_function_string: {}", &sx));
    match sx {
        "my_nickname" => rrc.game_data.my_nickname.to_owned(),
        "blink_or_not" => divnicknamemod::blink_or_not(rrc),
        "my_ws_uid" => format!("{}", rrc.game_data.my_ws_uid),
        "players_count" => format!("{} ", rrc.game_data.players.len() - 1),
        "game_name" => format!("{}", rrc.game_data.game_name),
        "group_id_joined" => group_id_joined(rrc),
        _ => {
            let x = format!("Error: Unrecognized call_function_string: {}", sx);
            logmod::debug_write(&x);
            x
        }
    }
}
/// html_templating functions for listeners
/// get a clone of the VdomWeak
pub fn call_listener(vdom: &dodrio::VdomWeak, rrc: &mut RootRenderingComponent, sx: String) {
    //logmod::debug_write(&format!("call_listener: {}", &sx));
    match sx.as_str() {
        "nickname_onkeyup" => {
            divnicknamemod::nickname_onkeyup(vdom);
        }
        "start_a_group_onclick" => {
            set_hash("#02");
        }
        "join_a_group_onclick" => {
            set_hash("#03");
        }
        "start_game_onclick" => {
            set_hash("#11");
        }
        "game_type_right_onclick" => {
            game_type_right_onclick(rrc, vdom);
        }
        "game_type_left_onclick" => {
            game_type_left_onclick(rrc, vdom);
        }
        "join_group_on_click" => {
            //find the group_id input element
            let group_id = get_input_value("input_group_id");
            set_hash(&format!("#04.{}", group_id));
        }
        _ => {
            let x = format!("Error: Unrecognized call_listener: {}", sx);
            logmod::debug_write(&x);
        }
    }
}

//TODO: html_templating functions that return a Node

pub fn game_type_right_onclick(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    let gmd = &rrc
        .game_data
        .games_metadata
        .as_ref()
        .unwrap()
        .vec_game_metadata;
    let mut last_name = gmd.last().unwrap().name.to_string();
    for x in gmd {
        if rrc.game_data.game_name.as_str() == last_name.as_str() {
            rrc.game_data.game_name = x.name.to_string();
            vdom.schedule_render();
            break;
        }
        last_name = x.name.to_string();
    }
}

pub fn game_type_left_onclick(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    let gmd = &rrc
        .game_data
        .games_metadata
        .as_ref()
        .unwrap()
        .vec_game_metadata;
    let mut last_name = gmd.first().unwrap().name.to_string();
    for x in gmd.iter().rev() {
        if rrc.game_data.game_name.as_str() == last_name.as_str() {
            rrc.game_data.game_name = x.name.to_string();
            vdom.schedule_render();
            break;
        }
        last_name = x.name.to_string();
    }
}
/// get value form input html element by id
fn get_input_value(id: &str) -> String {
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document(), "document");
    logmod::debug_write(&format!("before get_element_by_id: {}", id));
    let input_el = unwrap!(document.get_element_by_id(id));
    //logmod::debug_write("before dyn_into");
    let input_html_element = unwrap!(input_el.dyn_into::<web_sys::HtmlInputElement>(), "dyn_into");
    //return
    input_html_element.value()
}

/// fn for window.location. set_hash
fn set_hash(hash: &str) {
    let window = unwrap!(web_sys::window());
    let _x = window.location().set_hash(hash);
}

/// send a message to the first player
/// return the text for html template replace
pub fn group_id_joined(rrc: &RootRenderingComponent) -> String {
    let group_id = rrc.game_data.players.get(0).unwrap().ws_uid;
    let group_id = format!("{}", group_id);
    group_id
}
