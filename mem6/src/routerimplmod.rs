//! routerimplmod
//! The fill_rrc_local_route() function has specific code
//! to route from the url hash part to a
//! html_template file to fetch. The file name is written to rrc.local_route.  
//! This contains only generic code and can be made into a library.

use crate::*;
use unwrap::unwrap;
//use dodrio::VdomWeak;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;

impl routermod::Routing for dodrio::VdomWeak {
    fn test(&self) {
        let _x = "aaa";
    }
    /// Start the router. The second parameter is a reference to a function that
    /// deals with the specific routes. So the generic route code is isolated from the specific
    /// and can be made a library.
    fn start_router(&self) {
        let v3 = self.clone();
        let mut on_hash_change = routerimplmod::closure_on_hash_change(v3);
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

/// the specific code to route short_local_route to actual filenames to download
/// and later dodrio_templating replace
pub fn fill_rrc_local_route(
    local_route: String,
    rrc: &mut RootRenderingComponent,
    vdom: dodrio::VdomWeak,
) {
    if local_route == "#p02" {
        let vdom = vdom.clone();
        fetchgmod::async_fetch_game_config_request(rrc, &vdom);
        rrc.web_communication.local_route = "p02_start_a_group.html".to_owned();
    } else if local_route.starts_with("#p03") {
        rrc.game_data.my_player_number = 2;
        if local_route.contains('.') {
            let gr = routermod::get_url_param_in_hash_after_dot(&local_route);
            storagemod::save_group_id_string_to_local_storage(rrc, gr.to_string());
        } else {
            storagemod::load_group_id_string(rrc);
        }
        rrc.web_communication.local_route = "p03_join_a_group.html".to_owned();
    } else if local_route == "#p04" {
        statusjoinedmod::on_load_joined(rrc);
        rrc.web_communication.local_route = "p04_wait_to_start.html".to_owned();
    } else if local_route == "#p05" {
        rrc.web_communication.local_route = "p05_choose_game.html".to_owned();
    } else if local_route == "#p06" {
        rrc.web_communication.local_route = "p06_drink.html".to_owned();
    } else if local_route == "#p07" {
        rrc.web_communication.local_route = "p07_do_not_drink.html".to_owned();
    } else if local_route == "#p08" {
        rrc.web_communication.local_route = "p08_instructions.html".to_owned();
    } else if local_route == "#p11" {
        rrc.web_communication.local_route = "p11_gameboard.html".to_owned();
    } else if local_route == "#p21" {
        rrc.web_communication.local_route = "p21_menu.html".to_owned();
    } else if local_route == "#p31" {
        rrc.web_communication.local_route = "p31_debug_text.html".to_owned();
    } else {
        rrc.web_communication.local_route = "p01_start.html".to_owned();
    }
}

pub fn closure_on_hash_change(vdom: dodrio::VdomWeak) -> Box<dyn FnMut()> {
    // Callback fired whenever the URL hash fragment changes.
    // Keeps the rrc.web_communication.local_route in sync with the `#` fragment.
    Box::new(move || {
        let location = websysmod::window().location();
        let mut short_local_route = unwrap!(location.hash());
        if short_local_route.is_empty() {
            short_local_route = "index".to_owned();
        }
        // websysmod::debug_write("after .hash");
        wasm_bindgen_futures::spawn_local({
            let vdom = vdom.clone();
            async move {
                let _ = vdom
                    .with_component({
                        let vdom = vdom.clone();
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            // If the rrc local_route already matches the event's
                            // short_local_route, then there is nothing to do (ha). If they
                            // don't match, then we need to update the rrc' local_route
                            // and re-render.
                            if rrc.web_communication.local_route != short_local_route {
                                // all the specific routes are separated from the generic routing code
                                let v2 = vdom.clone();
                                fill_rrc_local_route(short_local_route, rrc, v2);
                                let url = rrc.web_communication.local_route.to_string();
                                // I cannot simply await here because this closure is not async
                                spawn_local(routermod::async_fetch_and_write_to_rrc_html_template(
                                    url, vdom,
                                ));
                            }
                        }
                    })
                    .await;
            }
        });
    })
}
