//windowmod.rs
//! helper functions for window, document, dom and web_sys

use crate::*;
//use mem6_common::*;
use unwrap::unwrap;
use wasm_bindgen::JsCast; // don't remove this. It is needed for dyn_into.
use rand::{Rng, rngs::SmallRng, SeedableRng};

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(windowmod::window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// get input element value string by id
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    // logmod::debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // logmod::debug_write("before dyn_into");
    let input_html_element = unwrap!(
        input_element.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    // logmod::debug_write("before value()");
    input_html_element.value()
}

/// save to local storage
pub fn save_to_local_storage(name: &str, value: &str) {
    let ls = unwrap!(unwrap!(windowmod::window().local_storage()));
    let _x = ls.set_item(name, value);
}

/// load string from local_storage
pub fn load_string_from_local_storage(name: &str, default_value: &str) -> String {
    let ls = unwrap!(unwrap!(windowmod::window().local_storage()));
    // return nickname
    unwrap!(ls.get_item(name)).unwrap_or(default_value.to_string())
}

/// load string from session_storage
pub fn load_string_from_session_storage(name: &str, default_value: &str) -> String {
    let ls = unwrap!(unwrap!(windowmod::window().session_storage()));
    let default_value_string = default_value.to_string();
    // return
    unwrap!(ls.get_item(name)).unwrap_or(default_value_string)
}

/// save my_ws_uid to session storage
pub fn save_string_to_session_storage(name: &str, value: &str) {
    let ls = unwrap!(unwrap!(windowmod::window().session_storage()));
    // session_storage saves only strings
    let _x = ls.set_item(name, value);
}

/// get a random number, min inclusive, max exclusive
pub fn get_random(min: usize, max: usize) -> usize {
    let mut rng = SmallRng::from_entropy();
    // return
    rng.gen_range(min, max)
}
