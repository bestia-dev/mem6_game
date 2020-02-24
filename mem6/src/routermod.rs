//! routermod - A simple `#`-fragment router for dodrio html templating  
//! This is the generic module. All the specifics for a website are isolated in the  
//! function &fill_rrc_local_route() passed as a parameter to start_router().  
//! The RootRenderingComponent struct must have the fields:  
//! rrc.web_communication.local_route - fill_rrc_local_route() will fills this field.
//!        It will be the name of the html template file to fetch  
//! rrc.web_communication.html_template - a string where the html_template is fetched from the server  

// TODO: how to write this without RootRenderingComponent
// write a Trait that has local route and html_template
use crate::*;

//use dodrio::VdomWeak;
use wasm_bindgen::{prelude::*, JsCast};
//use wasm_bindgen_futures::spawn_local;
use unwrap::unwrap;

/// trait intended to be added to VdomWeak
pub trait Routing {
    //region: specific code to be implemented
    fn start_router(&self);
    //endregion: specific code
    fn set_on_hash_change_callback(&self, mut on_hash_change: Box<dyn FnMut()>) {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.web_communication.local_route in sync with the `#` fragment.
        // Call it once to handle the initial `#` fragment.
        on_hash_change();

        // Now listen for hash changes forever.
        //
        // Note that if we ever intended to unmount our app, we would want to
        // provide a method for removing this router's event listener and cleaning
        // up after ourselves.
        #[allow(clippy::as_conversions)]
        let on_hash_change = Closure::wrap(on_hash_change);
        websysmod::window()
            .add_event_listener_with_callback("hashchange", on_hash_change.as_ref().unchecked_ref())
            .unwrap_throw();
        on_hash_change.forget();
    }
}

//region: generic trait code

/// get the first param after hash in local route after dot
/// example &p03.1234 -> 1234
pub fn get_url_param_in_hash_after_dot(short_local_route: &str) -> &str {
    let mut spl = short_local_route.split('.');
    unwrap!(spl.next());
    unwrap!(spl.next())
}
