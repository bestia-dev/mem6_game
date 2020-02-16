//! A simple `#`-fragment router for dodrio.

use crate::*;

use mem6_common::*;

use dodrio::VdomWeak;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use unwrap::unwrap;

/// Start the router.
pub fn start_router(vdom: VdomWeak) {
    // Callback fired whenever the URL hash fragment changes. Keeps the rrc.local_route
    // in sync with the `#` fragment.
    let on_hash_change = move || {
        let location = utilsmod::window().location();
        let mut local_route = unwrap!(location.hash());
        if local_route.is_empty() {
            local_route = "index".to_owned();
        }
        //logmod::debug_write("after .hash");
        wasm_bindgen_futures::spawn_local({
            let vdom = vdom.clone();
            async move {
                let _ = vdom
                    .with_component({
                        let vdom = vdom.clone();
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            // If the rrc local_route already matches the event's
                            // local_route, then there is nothing to do (ha). If they
                            // don't match, then we need to update the rrc' local_route
                            // and re-render.
                            if rrc.local_route != local_route {
                                if local_route == "#p02" {
                                    let vdom = vdom.clone();
                                    fetchgameconfigmod::async_fetch_game_config_request(rrc, &vdom);
                                    rrc.local_route = "p02_start_a_group.html".to_owned();
                                } else if local_route.starts_with("#p03") {
                                    if local_route.contains('.') {
                                        let group_id =
                                            get_url_param_in_hash_after_dot(&local_route);
                                        push_first_player_as_group_id(rrc, group_id);
                                    }
                                    rrc.local_route = "p03_join_a_group.html".to_owned();
                                } else if local_route.starts_with("#p04") {
                                    let group_id = get_url_param_in_hash_after_dot(&local_route);
                                    push_first_player_as_group_id(rrc, group_id);
                                    statusjoinedmod::on_load_joined(rrc);
                                    rrc.local_route = "p04_wait_to_start.html".to_owned();
                                } else if local_route == "#p05" {
                                    rrc.local_route = "p05_choose_game.html".to_owned();
                                } else if local_route == "#p06" {
                                    rrc.local_route = "p06_drink.html".to_owned();
                                } else if local_route == "#p07" {
                                    rrc.local_route = "p07_do_not_drink.html".to_owned();
                                } else if local_route == "#p08" {
                                    rrc.local_route = "p08_instructions.html".to_owned();
                                } else if local_route == "#p11" {
                                    rrc.local_route = "p11_gameboard.html".to_owned();
                                } else {
                                    rrc.local_route = "p01_start.html".to_owned();
                                }

                                let url = rrc.local_route.to_string();

                                //I cannot simply await here because this closure is not async
                                spawn_local(async_fetch_and_write_to_rrc_html_template(url, vdom));
                            }
                        }
                    })
                    .await;
            }
        });
    };

    // Call it once to handle the initial `#` fragment.
    on_hash_change();

    // Now listen for hash changes forever.
    //
    // Note that if we ever intended to unmount our app, we would want to
    // provide a method for removing this router's event listener and cleaning
    // up after ourselves.
    #[allow(clippy::as_conversions)]
    let on_hash_change = Closure::wrap(Box::new(on_hash_change) as Box<dyn FnMut()>);
    utilsmod::window()
        .add_event_listener_with_callback("hashchange", on_hash_change.as_ref().unchecked_ref())
        .unwrap_throw();
    on_hash_change.forget();
}

/// get the first param after hash in local route after dot
/// example &p04.1234 -> 1234
fn get_url_param_in_hash_after_dot(local_route: &str) -> &str {
    let mut spl = local_route.split('.');
    unwrap!(spl.next());
    unwrap!(spl.next())
}

/// add the first player as group_id so the msg can be sent to him
pub fn push_first_player_as_group_id(rrc: &mut RootRenderingComponent, group_id: &str) {
    let ws_uid = unwrap!(group_id.parse::<usize>());
    rrc.game_data.players.clear();
    rrc.game_data.players.push(Player {
        ws_uid,
        nickname: "FirstPlayer".to_string(),
        points: 0,
    });
    rrc.game_data.players_ws_uid = gamedatamod::prepare_players_ws_uid(&rrc.game_data.players);
    logmod::debug_write(&format!(
        "players_ws_uid: {}",
        &rrc.game_data.players_ws_uid
    ));
}

/// Fetch the html_template and save it in rrc.html_template  
/// The async fn for executor spawn_local.  
/// example how to use it in on_click:  
/// ```
/// .on("click", |_root, vdom, _event| {
///     let v2 = vdom;
///     //async executor spawn_local is the recommended for wasm
///     let url = "t1.html".to_owned();
///     //this will change the rrc.html_template eventually
///     spawn_local(async_fetch_and_write_to_rrc_html_template(url, v2));
/// })
/// ```
pub async fn async_fetch_and_write_to_rrc_html_template(url: String, vdom: VdomWeak) {
    logmod::debug_write(&format!("fetch {}", &url));
    let mut resp_body_text: String = fetchmod::async_spwloc_fetch_text(url).await;
    // update values in rrc is async.
    // I can await a fn call or an async block.
    async {
        unwrap!(
            vdom.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    //only the html inside the <body> </body>
                    let pos1 = resp_body_text.find("<body>").unwrap_or(0);
                    let pos2 = resp_body_text.find("</body>").unwrap_or(0);
                    if pos1 != 0 {
                        #[allow(clippy::integer_arithmetic)]
                        {
                            resp_body_text =
                                unwrap!(resp_body_text.get(pos1 + 6..pos2)).to_string();
                        }
                    }
                    //logmod::debug_write(&format!("body: {}", resp_body_text));
                    rrc.html_template = resp_body_text;
                }
            })
            .await
        );
        vdom.schedule_render();
    }
    .await;
}
