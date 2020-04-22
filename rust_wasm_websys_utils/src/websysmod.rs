// websysmod.rs
//! helper functions for web_sys, window, document, dom, console, local_storage, session_storage

// region: use
use web_sys::console;
use unwrap::unwrap;
use rand::{Rng, rngs::SmallRng, SeedableRng};
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{Request, RequestInit, Response};
use wasm_bindgen_futures::{JsFuture};
// endregion: use

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// get element by id
pub fn get_element_by_id(element_id: &str) -> web_sys::Element {
    let document = unwrap!(window().document());
    unwrap!(document.get_element_by_id(element_id))
}

/// get input element value string by id
pub fn get_input_element_value_string_by_id(element_id: &str) -> String {
    // debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    // debug_write("before value()");
    input_html_element.value()
}

/// get input element value usize by id
pub fn get_input_element_value_usize_by_id(element_id: &str) -> usize {
    let receiver_ws_uid = get_input_element_value_string_by_id(element_id);
    unwrap!(receiver_ws_uid.parse::<usize>())
}

/// get input element value string by id
pub fn set_input_element_value_string_by_id(element_id: &str, value: &str) {
    // debug_write("before get_element_by_id");
    let input_element = get_element_by_id(element_id);
    // debug_write("before dyn_into");
    let input_html_element = unwrap!(input_element.dyn_into::<web_sys::HtmlInputElement>());
    // debug_write("before value()");
    input_html_element.set_value(value);
}

/// get input element value string by id
pub fn get_image_element_by_id(element_id: &str) -> web_sys::HtmlImageElement {
    let img_element = get_element_by_id(element_id);
    //return
    unwrap!(img_element.dyn_into::<web_sys::HtmlImageElement>())
}

/// get input element value string by id
pub fn get_audio_element_by_id(element_id: &str) -> web_sys::HtmlAudioElement {
    let audio_element = get_element_by_id(element_id);
    //return
    unwrap!(audio_element.dyn_into::<web_sys::HtmlAudioElement>())
}

/// save to local storage
pub fn save_to_local_storage(name: &str, value: &str) {
    let ls = unwrap!(unwrap!(window().local_storage()));
    let _x = ls.set_item(name, value);
}

/// load string from local_storage
pub fn load_string_from_local_storage(name: &str, default_value: &str) -> String {
    let ls = unwrap!(unwrap!(window().local_storage()));
    // return nickname
    unwrap!(ls.get_item(name)).unwrap_or(default_value.to_string())
}

/// load string from session_storage
pub fn load_string_from_session_storage(name: &str, default_value: &str) -> String {
    let ls = unwrap!(unwrap!(window().session_storage()));
    let default_value_string = default_value.to_string();
    // return
    unwrap!(ls.get_item(name)).unwrap_or(default_value_string)
}

/// save string to session storage
pub fn save_string_to_session_storage(name: &str, value: &str) {
    let ls = unwrap!(unwrap!(window().session_storage()));
    // session_storage saves only strings
    let _x = ls.set_item(name, value);
}

/// get a random number, min inclusive, max exclusive
pub fn get_random(min: usize, max: usize) -> usize {
    let mut rng = SmallRng::from_entropy();
    // return
    rng.gen_range(min, max)
}

// region: fetch

/// fetch in Rust with async await for executor spawn_local()
/// return the response as String. Any error will panic.
pub async fn fetch_response(url: String) -> String {
    // Request init
    let mut opts = RequestInit::new();
    opts.method("GET");
    let request = unwrap!(Request::new_with_str_and_init(&url, &opts));
    // log1("before fetch");
    let resp_jsvalue = unwrap!(JsFuture::from(window().fetch_with_request(&request)).await);
    // log1("after fetch");
    let resp: Response = unwrap!(resp_jsvalue.dyn_into());
    // log1("before text()");
    let text_jsvalue = unwrap!(JsFuture::from(unwrap!(resp.text())).await);
    let txt_response: String = unwrap!(text_jsvalue.as_string());
    // debug_write(&txt_response);
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
    unwrap!(JsFuture::from(window().fetch_with_request(&request)).await);
}

// endregion:fetch

/// get url and hash from window.location
#[must_use]
pub fn get_url_and_hash() -> (String, String) {
    // find out URL
    let mut location_href = unwrap!(window().location().href());
    // without /index.html
    location_href = location_href.to_lowercase().replace("index.html", "");
    //debug_write(&format!("location_href: {}", &location_href));

    // split it by # hash
    let cl = location_href.clone();
    let mut spl = cl.split('#');
    location_href = unwrap!(spl.next()).to_string();
    let href_hash = spl.next().unwrap_or("").to_string();

    //debug_write(&format!("location_href: {}", &location_href));
    //debug_write(&format!("href_hash: {}", &href_hash));
    (location_href, href_hash)
}

/// play sound mp3 from src file
pub fn play_sound(src: &str) {
    let audio_element = unwrap!(web_sys::HtmlAudioElement::new_with_src(src));
    let _x = unwrap!(audio_element.play());
}

/// debug write into session_storage
pub fn debug_write(text: &str) {
    // writing to the console is futile for mobile phones
    // I must write it on the UI.
    // so I must access this string from the UI renderer
    add_to_begin_of_debug_text(text);
    console::log_1(&JsValue::from_str(text));
}

/// string of now time
pub fn now_string() -> String {
    let now = js_sys::Date::new_0();
    // return
    format!(
        "{:02}:{:02}.{:03}",
        now.get_minutes(),
        now.get_seconds(),
        now.get_milliseconds()
    )
}

/// fn open new local page with #
/// and push to history
pub fn open_new_local_page_push_to_history(hash: &str) {
    let _x = window().location().assign(hash);
}

/// fn open new tab
pub fn open_new_tab(url: &str) {
    let _w = window().open_with_url_and_target(url, "_blank");
}
// region: debug_text
/// get debug text from session storage
pub fn get_debug_text() -> String {
    load_string_from_session_storage("debug_text", "")
}

/// add to begin of debug text
pub fn add_to_begin_of_debug_text(text: &str) {
    let mut debug_text = format!("{}: {}\n{}", now_string(), text, get_debug_text());
    utf8_truncate(&mut debug_text, 800);
    save_string_to_session_storage("debug_text", &debug_text);
}

/// utf8 truncate
fn utf8_truncate(input: &mut String, max_size: usize) {
    let mut utf8_max_size = input.len();
    if utf8_max_size >= max_size {
        {
            let mut char_iter = input.char_indices();
            while utf8_max_size >= max_size {
                utf8_max_size = match char_iter.next_back() {
                    Some((index, _)) => index,
                    None => 0,
                };
            }
        } // Extra {} wrap to limit the immutable borrow of char_indices()
        input.truncate(utf8_max_size);
    }
}

/// prepare json string for one ws receiver
pub fn prepare_json_msg_receivers_for_one(receiver_ws_uid: usize) -> String {
    let mut vec_msg_receivers = Vec::new();
    vec_msg_receivers.push(receiver_ws_uid);
    // return
    unwrap!(serde_json::to_string(&vec_msg_receivers))
}

// endregion: debug_text
