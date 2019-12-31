// sessionstoragemod.rs
//! for debugging texts accessible everywhere

//region: use
use crate::logmod;

use unwrap::unwrap;
//endregion

///add to begin of debug text
pub fn add_to_begin_of_debug_text(text: &str) {
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.session_storage()));
    let mut debug_text = format!("{}: {}\n{}", logmod::now_string(), text, get_debug_text());
    utf8_truncate(&mut debug_text, 800);
    let _x = ls.set_item("debug_text", &debug_text);
}

///utf8 truncate
fn utf8_truncate(input: &mut String, maxsize: usize) {
    let mut utf8_maxsize = input.len();
    if utf8_maxsize >= maxsize {
        {
            let mut char_iter = input.char_indices();
            while utf8_maxsize >= maxsize {
                utf8_maxsize = match char_iter.next_back() {
                    Some((index, _)) => index,
                    None => 0,
                };
            }
        } // Extra {} wrap to limit the immutable borrow of char_indices()
        input.truncate(utf8_maxsize);
    }
}

///get debug text from session storage
pub fn get_debug_text() -> String {
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.session_storage()));
    let empty1 = "".to_string();
    //return 
    unwrap!(ls.get_item("debug_text")).unwrap_or(empty1)
}

///save my_ws_uid to session storage
pub fn save_my_ws_uid(my_ws_uid: usize) {
    //save it only on smartphones. The desktop browser I use for debugging in multiple tabs.
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.session_storage()));
    //sessionstorage saves only strings
    let _x = ls.set_item("my_ws_uid", &format!("{}", my_ws_uid));
}

///load my_ws_uid from session storage
pub fn load_my_ws_uid() -> usize {
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.session_storage()));
    //sessionstorage saves only strings
    let str_uid = unwrap!(ls.get_item("my_ws_uid")).unwrap_or_else(|| "0".to_string());
    //return my_ws_uid
    let my_ws_uid = unwrap!(str_uid.parse::<usize>());
    my_ws_uid
}
