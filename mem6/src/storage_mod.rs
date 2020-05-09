//! storage_mod.rs
//! local_storage for nickname and group_id
//! session_storage for ws_uid and debug_text

use crate::*;
use wasm_bindgen::JsCast; // don't remove this. It is needed for dyn_into.
use unwrap::unwrap;
use web_sys::{Event,KeyboardEvent};

// region: my_ws_uid
/// save my_ws_uid to session storage so we can restart the game and preserve the ws_uid
pub fn save_my_ws_uid(my_ws_uid: usize) {
    websysmod::save_string_to_session_storage("my_ws_uid", &format!("{}", my_ws_uid))
}

/// load my_ws_uid from session storage
pub fn load_my_ws_uid() -> usize {
    // session_storage saves only strings
    let str_uid = websysmod::load_string_from_session_storage("my_ws_uid", "0");
    let my_ws_uid = unwrap!(str_uid.parse::<usize>());
    // return my_ws_uid
    my_ws_uid
}
// endregion: my_ws_uid

// region: nickname
/// save on every key stroke
pub fn nickname_onkeyup(rrc: &mut RootRenderingComponent, event: Event) {
    // websysmod::debug_write("on key up");
    let keyboard_event = unwrap!(event.dyn_into::<KeyboardEvent>());
    // websysmod::debug_write(&keyboard_event.key());
    if keyboard_event.key() == "Enter" {
        // open page start group
        html_template_impl_mod::open_new_local_page("#p02");
    } else {
        save_nickname_to_local_storage(rrc);
    }
    // vdom.schedule_render();
}

/// save nickname from html input elements to local storage and rrc
pub fn save_nickname_to_local_storage(rrc: &mut RootRenderingComponent) {
    let nickname = websysmod::get_input_element_value_string_by_id("input_nickname");
    websysmod::save_to_local_storage("nickname", &nickname);
    websysmod::debug_write(&format!("save nickname to local storage: {}", &nickname));

    rrc.game_data.my_nickname = nickname.clone();
    // change it also in players, if the player exists
    if rrc.game_data.my_player_number < rrc.game_data.players.len() {
        rrc.game_data.my_player_mut().nickname = nickname;
    }
}

/// load nickname from local storage
pub fn load_nickname() -> String {
    // return nickname
    websysmod::load_string_from_local_storage("nickname", "")
}

/// if there is already a nickname don't blink
pub fn blink_or_not_nickname(rrc: &RootRenderingComponent) -> String {
    if rrc.game_data.my_nickname.is_empty() {
        "blink".to_owned()
    } else {
        "".to_owned()
    }
}

// endregion: nickname

// region: group_id
/// group id key stroke
pub fn group_id_onkeyup(rrc: &mut RootRenderingComponent, event: Event) {
    // websysmod::debug_write("on key up");
    let keyboard_event = unwrap!(event.dyn_into::<KeyboardEvent>());
    // websysmod::debug_write(&keyboard_event.key());
    if keyboard_event.key() == "Enter" {
        // open page start group
        html_template_impl_mod::open_new_local_page("#p04");
    } else {
        save_group_id_to_local_storage(rrc);
    }
}

/// save group_id from html input elements to local storage and rrc
pub fn save_group_id_to_local_storage(rrc: &mut RootRenderingComponent) {
    let group_id_string = websysmod::get_input_element_value_string_by_id("input_group_id");
    save_group_id_string_to_local_storage(rrc, &group_id_string);
}

/// save group_id from html input elements to local storage and rrc
pub fn save_group_id_string_to_local_storage(
    rrc: &mut RootRenderingComponent,
    group_id_string: &str,
) {
    set_group_id(rrc, group_id_string);
    websysmod::save_to_local_storage("group_id", group_id_string);
}

/// load group_id from local storage
pub fn load_group_id_string(rrc: &mut RootRenderingComponent) -> String {
    let group_id_string = websysmod::load_string_from_local_storage("group_id", "");
    set_group_id(rrc, &group_id_string);
    // return
    group_id_string
}

/// there are 3 places that must be managed (plus the local_storage)
pub fn set_group_id(rrc: &mut RootRenderingComponent, group_id_string: &str) {
    rrc.game_data.group_id = group_id_string.parse::<usize>().unwrap_or(0);
    // change it also in players[0]
    #[allow(clippy::indexing_slicing)]
    // cannot panic because player[0] must exist
    {
        rrc.game_data.players[0].ws_uid = rrc.game_data.group_id;
    }
    // on any change in players the msg_receivers_json must be constructed
    rrc.web_data.msg_receivers_json = rrc.game_data.prepare_json_msg_receivers();
}
// endregion: group_id
