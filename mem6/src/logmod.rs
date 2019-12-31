// logmod.rs
//! logging in wasm

use crate::sessionstoragemod;

//use web_sys::console;
//use wasm_bindgen::prelude::*;

/*
///simple console write with a string
pub fn console_log(x: &str) {
    console::log_1(&JsValue::from_str(x));
}
*/

/*
///simple console write with JsValue
pub fn log1_jsvalue(x: &JsValue) {
    console::log_1(x);
}
*/

/// debug write into sessionstorage
pub fn debug_write(text: &str) {
    //writing to the console is futile for mobile phones
    //I must write it on the UI.
    //so I must access this string from the UI rendere
    sessionstoragemod::add_to_begin_of_debug_text(text);
}

///string of now time
pub fn now_string() -> String {
    let now = js_sys::Date::new_0();
    //return
    format!(
        "{:02}:{:02}.{:03}",
        now.get_minutes(),
        now.get_seconds(),
        now.get_milliseconds()
    )
}
