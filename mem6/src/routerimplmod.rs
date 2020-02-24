//! routerimplmod
//! The fill_rrc_local_route() function has specific code
//! to route from the url hash part to a
//! html_template file to fetch. The file name is written to rrc.local_route.  
//! This contains only generic code and can be made into a library.

use crate::*;
use unwrap::unwrap;
use dodrio::VdomWeak;
//use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;

impl routermod::Routing for dodrio::VdomWeak {
    /// Start the router. The second parameter is a reference to a function that
    /// deals with the specific routes. So the generic route code is isolated from the specific
    /// and can be made a library.
    fn start_router(&self) {
        let v3 = self.clone();
        let on_hash_change = Self::closure_on_hash_change(v3);
        self.set_on_hash_change_callback(on_hash_change);
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

pub fn closure_2(
    vdom: dodrio::VdomWeak,
    short_local_route: String,
) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static> {
    // Callback fired whenever the URL hash fragment changes.
    // Keeps the rrc.web_communication.local_route in sync with the `#` fragment.
    Box::new(move |root| {
        let short_local_route = short_local_route.clone();
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
            let v3 = vdom.clone();
            spawn_local(async_fetch_and_write_to_rrc_html_template(url, v3));
        }
    })
}

/// Fetch the html_template and save it in rrc.web_communication.html_template  
/// The async fn for executor spawn_local.  
/// example how to use it in on_click:  
/// ```
/// .on("click", |_root, vdom, _event| {
///     let v2 = vdom;
///     // async executor spawn_local is the recommended for wasm
///     let url = "t1.html".to_owned();
///     // this will change the rrc.web_communication.html_template eventually
///     spawn_local(async_fetch_and_write_to_rrc_html_template(url, v2));
/// })
/// ```
/// async fn cannot be trait fn as of 24.2.2020 cargo version 1.41.0
pub async fn async_fetch_and_write_to_rrc_html_template(url: String, vdom: VdomWeak) {
    websysmod::debug_write(&format!("fetch {}", &url));
    let mut resp_body_text: String = websysmod::async_spwloc_fetch_text(url).await;
    // update values in rrc is async.
    // I can await a fn call or an async block.
    async {
        unwrap!(
            vdom.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    // only the html inside the <body> </body>
                    let pos1 = resp_body_text.find("<body>").unwrap_or(0);
                    let pos2 = resp_body_text.find("</body>").unwrap_or(0);
                    if pos1 != 0 {
                        #[allow(clippy::integer_arithmetic)]
                        {
                            resp_body_text =
                                unwrap!(resp_body_text.get(pos1 + 6..pos2)).to_string();
                        }
                    }
                    // websysmod::debug_write(&format!("body: {}", resp_body_text));
                    rrc.web_communication.html_template = resp_body_text;
                }
            })
            .await
        );
        vdom.schedule_render();
    }
    .await;
}
