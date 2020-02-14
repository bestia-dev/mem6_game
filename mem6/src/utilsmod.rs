//utilsmod.rs
//! small generic helper functions
use unwrap::unwrap;

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}
