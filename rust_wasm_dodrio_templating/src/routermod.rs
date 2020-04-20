//! routermod - A simple `#`-fragment router for dodrio html templating  
//! This is the generic module. All the specifics for a website are isolated in the  
//! module routerimplmod.  

use crate::*;

use dodrio::VdomWeak;
use wasm_bindgen::{prelude::*, JsCast};
use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;

/// trait intended to be added to VdomWeakWrapper
pub trait Routing {
    // region: specific code to be implemented on Router struct
    fn get_local_route(root: &mut dyn dodrio::RootRender) -> &str;
    fn update_local_route(
        local_route: String,
        root: &mut dyn dodrio::RootRender,
        vdom: VdomWeak,
    ) -> String;
    fn update_html_template_and_sub_templates(
        resp_body_text: String,
    ) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static>;
    // endregion: specific code

    // region:generic code (boilerplate)
    /// Start the router.
    fn start_router(&self, vdom: VdomWeak) {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.web_data.local_route in sync with the `#` fragment.
        let on_hash_change = Box::new(move || {
            let location = websysmod::window().location();
            let mut short_local_route = unwrap!(location.hash());
            if short_local_route.is_empty() {
                short_local_route = "index".to_owned();
            }
            // websysmod::debug_write("after .hash");
            wasm_bindgen_futures::spawn_local({
                let vdom_on_next_tick = vdom.clone();
                async move {
                    let _ = vdom_on_next_tick
                        .with_component({
                            let vdom = vdom_on_next_tick.clone();
                            // Callback fired whenever the URL hash fragment changes.
                            // Keeps the rrc.web_data.local_route in sync with the `#` fragment.
                            move |root| {
                                let short_local_route = short_local_route.clone();
                                // If the rrc local_route already matches the event's
                                // short_local_route, then there is nothing to do (ha). If they
                                // don't match, then we need to update the rrc' local_route
                                // and re-render.
                                if Self::get_local_route(root) != short_local_route {
                                    // the function that recognizes routes and urls
                                    let url = Self::update_local_route(
                                        short_local_route,
                                        root,
                                        vdom.clone(),
                                    );
                                    // I cannot simply await here because this closure is not async
                                    spawn_local({
                                        let vdom_on_next_tick = vdom.clone();
                                        async move {
                                            //websysmod::debug_write(&format!("fetch {}", &url));
                                            let resp_body_text: String =
                                                websysmod::async_spwloc_fetch_text(url).await;
                                            // update values in rrc is async.
                                            unwrap!(
                                                vdom_on_next_tick
                                                    .with_component({
                                                        Self::update_html_template_and_sub_templates(resp_body_text)
                                                    })
                                                    .await
                                            );
                                            vdom.schedule_render();
                                        }
                                    });
                                }
                            }
                        })
                        .await;
                }
            });
        });
        self.set_on_hash_change_callback(on_hash_change);
    }

    fn set_on_hash_change_callback(&self, mut on_hash_change: Box<dyn FnMut()>) {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.web_data.local_route in sync with the `#` fragment.
        // Call it once to handle the initial `#` fragment.
        on_hash_change();

        // Now listen for hash changes forever.
        //
        // Note that if we ever intended to unmount our app, we would want to
        // provide a method for removing this router's event listener and cleaning
        // up after ourselves.
        let on_hash_change = Closure::wrap(on_hash_change);
        websysmod::window()
            .add_event_listener_with_callback("hashchange", on_hash_change.as_ref().unchecked_ref())
            .unwrap_throw();
        on_hash_change.forget();
    }

    // endregion:generic
}

/// get the first param after hash in local route after dot
/// example &p03.1234 -> 1234
pub fn get_url_param_in_hash_after_dot(short_local_route: &str) -> &str {
    let mut spl = short_local_route.split('.');
    unwrap!(spl.next());
    unwrap!(spl.next())
}

/// only the html between the <body> </body>
/// it must be a SINGLE root node
pub fn between_body_tag(resp_body_text: &str) -> String {
    let pos1 = resp_body_text.find("<body>").unwrap_or(0);
    let pos2 = resp_body_text.find("</body>").unwrap_or(0);
    // return
    if pos1 == 0 {
        resp_body_text.to_string()
    } else {
        #[allow(clippy::integer_arithmetic)]
        {
            unwrap!(resp_body_text.get(pos1 + 6..pos2)).to_string()
        }
    }
}
