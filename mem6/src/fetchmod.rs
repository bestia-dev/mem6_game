// fetchmod.rs
//region description
//! ## Async world
//! With the new async/.await syntax in Rust it is now very easy to write async code.
//! It is important to be careful to NOT write sync code after async code.
//! It can be confusing that the sync code will execute before the async code !
//! Webassembly is basically javascript and uses the recommended executor spawn_local() or future_to_promise().
//! Async cannot use any references to the stack because it is executed in another timeline.
//endregion

//region: use
use crate::*;

use unwrap::unwrap;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen_futures::{JsFuture};
//endregion

/// fetch in Rust with async await for executor spawn_local()
/// return the response as JsValue. Any error will panic.
pub async fn async_spwloc_fetch_text(url: String) -> String {
    //Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    let resp_jsvalue =
        unwrap!(JsFuture::from(utilsmod::window().fetch_with_request(&request)).await);
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    let resp_body_text = unwrap!(JsFuture::from(unwrap!(resp.text())).await);
    //logmod::debug_write(&unwrap!(JsValue::as_string(&resp_body_text)));
    //returns response as String
    unwrap!(JsValue::as_string(&resp_body_text))
}

/// fetch in Rust with async await for executor spawn_local()
/// return the response as String. Any error will panic.
pub async fn fetch_response(url: String) -> String {
    //Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    //log1("before fetch");
    let resp_jsvalue =
        unwrap!(JsFuture::from(utilsmod::window().fetch_with_request(&request)).await);
    //log1("after fetch");
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    //log1("before text()");
    let text_jsvalue = unwrap!(JsFuture::from(unwrap!(resp.text())).await);
    let txt_response: String = unwrap!(text_jsvalue.as_string());
    //logmod::debug_write(&txt_response);
    //returns response as String
    txt_response
}

/// fetch only, so it goes in cache
pub async fn fetch_only(url: String) {
    //Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    //log1("before fetch");
    unwrap!(JsFuture::from(utilsmod::window().fetch_with_request(&request)).await);
}
