// javascriptimportmod.rs
//! one single module to import javascript functions and objects
//! this are examples how to call a javascript function from rust

use wasm_bindgen::prelude::*;

///in the block extern "C" are the descriptions of imported javascript
#[wasm_bindgen(module = "/js/javascriptdofullscreen.js")]
extern "C" {
    /// the name of the extern javascript function
    #[allow(clippy::missing_docs_in_private_items)]
    fn javascriptdofullscreen();
}

///in the block extern "C" are the descriptions of imported javascript
#[wasm_bindgen(module = "/js/javascriptismobiledevice.js")]
extern "C" {
    /// the name of the extern javascript function
    #[allow(clippy::missing_docs_in_private_items)]
    fn javascriptismobiledevice() -> bool;
}

///do full screen function - imported from javascript
pub fn do_fullscreen() {
    javascriptdofullscreen()
}

///is mobile device function - imported from javascript
pub fn is_mobile_device() -> bool {
    javascriptismobiledevice()
}
