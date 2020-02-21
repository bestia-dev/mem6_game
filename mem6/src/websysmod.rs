//websysmod.rs
//! helper functions for web_sys, window, document, dom and web_sys

use crate::*;
//use mem6_common::*;
use unwrap::unwrap;
use rand::{Rng, rngs::SmallRng, SeedableRng};
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen_futures::{JsFuture};

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(websysmod::window().document());
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
    let ls = unwrap!(unwrap!(websysmod::window().local_storage()));
    let _x = ls.set_item(name, value);
}

/// load string from local_storage
pub fn load_string_from_local_storage(name: &str, default_value: &str) -> String {
    let ls = unwrap!(unwrap!(websysmod::window().local_storage()));
    // return nickname
    unwrap!(ls.get_item(name)).unwrap_or(default_value.to_string())
}

/// load string from session_storage
pub fn load_string_from_session_storage(name: &str, default_value: &str) -> String {
    let ls = unwrap!(unwrap!(websysmod::window().session_storage()));
    let default_value_string = default_value.to_string();
    // return
    unwrap!(ls.get_item(name)).unwrap_or(default_value_string)
}

/// save string to session storage
pub fn save_string_to_session_storage(name: &str, value: &str) {
    let ls = unwrap!(unwrap!(websysmod::window().session_storage()));
    // session_storage saves only strings
    let _x = ls.set_item(name, value);
}

/// get a random number, min inclusive, max exclusive
pub fn get_random(min: usize, max: usize) -> usize {
    let mut rng = SmallRng::from_entropy();
    // return
    rng.gen_range(min, max)
}

//region: fetch
/// fetch in Rust with async await for executor spawn_local()
/// return the response as JsValue. Any error will panic.
pub async fn async_spwloc_fetch_text(url: String) -> String {
    // Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    let resp_jsvalue =
        unwrap!(JsFuture::from(websysmod::window().fetch_with_request(&request)).await);
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    let resp_body_text = unwrap!(JsFuture::from(unwrap!(resp.text())).await);
    // logmod::debug_write(&unwrap!(JsValue::as_string(&resp_body_text)));
    // returns response as String
    unwrap!(JsValue::as_string(&resp_body_text))
}

/// fetch in Rust with async await for executor spawn_local()
/// return the response as String. Any error will panic.
pub async fn fetch_response(url: String) -> String {
    // Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    // log1("before fetch");
    let resp_jsvalue =
        unwrap!(JsFuture::from(websysmod::window().fetch_with_request(&request)).await);
    // log1("after fetch");
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    // log1("before text()");
    let text_jsvalue = unwrap!(JsFuture::from(unwrap!(resp.text())).await);
    let txt_response: String = unwrap!(text_jsvalue.as_string());
    // logmod::debug_write(&txt_response);
    // returns response as String
    txt_response
}

/// fetch only, so it goes in cache
pub async fn fetch_only(url: String) {
    // Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    // log1("before fetch");
    unwrap!(JsFuture::from(websysmod::window().fetch_with_request(&request)).await);
}

//endregion:fetch
