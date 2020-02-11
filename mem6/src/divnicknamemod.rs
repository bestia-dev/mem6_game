// divnicknamemod.rs
//! load and save nickname

#![allow(clippy::panic)]

//region: use
use crate::*;

//use unwrap::unwrap;
use unwrap::unwrap;
use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
                          //endregion

///save nickname from html input elements to local storage and rrc
pub fn save_nickname_to_localstorage(rrc: &mut RootRenderingComponent) {
    let window = unwrap!(web_sys::window(), "window");
    let document = unwrap!(window.document(), "document");

    //logmod::debug_write("before get_element_by_id");
    let input_nickname = unwrap!(document.get_element_by_id("input_nickname"));
    //logmod::debug_write("before dyn_into");
    let input_html_element_nickname = unwrap!(
        input_nickname.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    //logmod::debug_write("before value()");
    let nickname_string = input_html_element_nickname.value();
    //logmod::debug_write("before as_str");
    let nickname = nickname_string.as_str();
    //logmod::debug_write(nickname);

    let ls = unwrap!(unwrap!(window.local_storage()));
    let _x = ls.set_item("nickname", nickname);

    //To change the data in rrc I must use the future `vdom.with_component`
    //it will be executed at the next tick to avoid concurrent data races.
    rrc.game_data.my_nickname = nickname_string;
}

///load nickname from local storage
pub fn load_nickname() -> String {
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.local_storage()));
    let empty1 = "nickname".to_string();
    //return nickname
    unwrap!(ls.get_item("nickname")).unwrap_or(empty1)
}

/// if there is already a nickname don't blink
pub fn blink_or_not_nickname(rrc: &RootRenderingComponent) -> String {
    if rrc.game_data.my_nickname == "nickname" {
        "blink".to_owned()
    } else {
        "".to_owned()
    }
}

/// save on every key stroke
pub fn nickname_onkeyup(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    //logmod::debug_write("on key up");
    save_nickname_to_localstorage(rrc);
    vdom.schedule_render();
}
