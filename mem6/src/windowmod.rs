//windowmod.rs
//! helper functions for window, document, dom and web_sys

use crate::*;
use mem6_common::*;
use unwrap::unwrap;
use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    let document = unwrap!(window().document(), "document");

    //logmod::debug_write("before get_element_by_id");
    let input_group_id = unwrap!(document.get_element_by_id(element_id));
    //logmod::debug_write("before dyn_into");
    let input_html_element_group_id = unwrap!(
        input_group_id.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    //logmod::debug_write("before value()");
    input_html_element_group_id.value()
}
