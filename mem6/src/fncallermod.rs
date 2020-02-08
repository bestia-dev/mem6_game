//! fncallermod  

use crate::*;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use mem6_common::*;

use unwrap::unwrap;
use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use dodrio::builder::*;
use typed_html::dodrio;

/// html_templating functions that return a String
#[allow(clippy::needless_return, clippy::integer_arithmetic)]
pub fn call_function_string(rrc: &RootRenderingComponent, sx: &str) -> String {
    //logmod::debug_write(&format!("call_function_string: {}", &sx));
    match sx {
        "my_nickname" => rrc.game_data.my_nickname.to_owned(),
        "blink_or_not" => divnicknamemod::blink_or_not(rrc),
        "my_ws_uid" => format!("{}", rrc.game_data.my_ws_uid),
        "players_count" => format!("{} ", rrc.game_data.players.len() - 1),
        "game_name" => rrc.game_data.game_name.to_string(),
        "group_id_joined" => group_id_joined(rrc),
        "gameboard_btn" => {
            //different class depend on status
            "btn".to_owned()
        }
        "card_moniker_first" => {
            return unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                .card_moniker
                .get(
                    unwrap!(rrc
                        .game_data
                        .card_grid_data
                        .get(rrc.game_data.card_index_of_first_click))
                    .card_number_and_img_src
                ))
            .to_string();
        }
        "card_moniker_second" => {
            return unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                .card_moniker
                .get(
                    unwrap!(rrc
                        .game_data
                        .card_grid_data
                        .get(rrc.game_data.card_index_of_second_click))
                    .card_number_and_img_src
                ))
            .to_string();
        }
        "my_points" => {
            return format!(
                "{} ",
                unwrap!(rrc
                    .game_data
                    .players
                    .get(rrc.game_data.my_player_number - 1))
                .points,
            );
        }
        "player_turn" => {
            return unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn - 1))
                .nickname
                .to_string();
        }
        _ => {
            let x = format!("Error: Unrecognized call_function_string: {}", sx);
            logmod::debug_write(&x);
            x
        }
    }
}
/// html_templating functions for listeners
/// get a clone of the VdomWeak
pub fn call_listener(vdom: &dodrio::VdomWeak, rrc: &mut RootRenderingComponent, sx: &str) {
    //logmod::debug_write(&format!("call_listener: {}", &sx));
    match sx {
        "nickname_onkeyup" => {
            divnicknamemod::nickname_onkeyup(vdom);
        }
        "start_a_group_onclick" => {
            open_new_local_page("#p02");
        }
        "join_a_group_onclick" => {
            open_new_local_page("#p03");
        }
        "start_game_onclick" => {
            statusgamedatainitmod::on_click_start_game(rrc);
            //async fetch all imgs and put them in service worker cache
            fetchallimgsforcachemod::fetch_all_img_for_cache_request(rrc);
            //endregion
            vdom.schedule_render();
            //logmod::debug_write(&format!("start_game_onclick players: {:?}",rrc.game_data.players));
            open_new_local_page("#p11");
        }
        "game_type_right_onclick" => {
            game_type_right_onclick(rrc, vdom);
        }
        "game_type_left_onclick" => {
            game_type_left_onclick(rrc, vdom);
        }
        "join_group_on_click" => {
            //find the group_id input element
            let group_id = get_input_value("input_group_id");
            open_new_local_page(&format!("#p04.{}", group_id));
        }
        "drink_end" => {
            //send a msg to end drinking to all players
            logmod::debug_write(&format!("MsgDrinkEnd send{}", ""));
            websocketcommunicationmod::ws_send_msg(
                &rrc.game_data.ws,
                &WsMessage::MsgDrinkEnd {
                    my_ws_uid: rrc.game_data.my_ws_uid,
                    players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                },
            );
            //if all the cards are permanently up, this is the end of the game
            //logmod::debug_write("if is_all_permanently(rrc)");
            if status2ndcardmod::is_all_permanently(rrc) {
                logmod::debug_write("yes");
                statusgameovermod::on_msg_game_over(rrc);
                //send message
                websocketcommunicationmod::ws_send_msg(
                    &rrc.game_data.ws,
                    &WsMessage::MsgGameOver {
                        my_ws_uid: rrc.game_data.my_ws_uid,
                        players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                    },
                );
            } else {
                statustaketurnmod::on_click_take_turn(rrc, vdom);
            }
            //end the drink dialog
            open_new_local_page("#p11");
        }
        _ => {
            let x = format!("Error: Unrecognized call_listener: {}", sx);
            logmod::debug_write(&x);
        }
    }
}

/// html_templating functions that return a Node
#[allow(clippy::needless_return)]
pub fn call_function_node<'a>(rrc: &RootRenderingComponent, bump: &'a Bump, sx: &str) -> Node<'a> {
    //logmod::debug_write(&format!("call_function_node: {}", &sx));
    match sx {
        "div_grid_container" => {
            //what is the game_status now?
            //logmod::debug_write(&format!("game status: {}", rrc.game_data.game_status));
            let max_grid_size = divgridcontainermod::max_grid_size(rrc);
            return divgridcontainermod::div_grid_container(rrc, bump, &max_grid_size);
        }
        "div_player_action" => {
            let node = divplayeractionsmod::div_player_actions_from_game_status(rrc, bump);
            return node;
        }
        _ => {
            let node = dodrio!(bump,
            <h2  >
                {vec![text(bumpalo::format!(in bump, "Error: Unrecognized call_function_node: {}", sx).into_bump_str())]}
            </h2>
            );
            return node;
        }
    }
}

/// the arrow to the right
pub fn game_type_right_onclick(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    let gmd = &unwrap!(rrc.game_data.games_metadata.as_ref()).vec_game_metadata;
    let mut last_name = unwrap!(gmd.last()).name.to_string();
    for x in gmd {
        if rrc.game_data.game_name.as_str() == last_name.as_str() {
            rrc.game_data.game_name = x.name.to_string();
            vdom.schedule_render();
            break;
        }
        last_name = x.name.to_string();
    }
    fetchgameconfigmod::async_fetch_game_config_request(rrc, vdom);
}
/// left arrow button
pub fn game_type_left_onclick(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    let gmd = &unwrap!(rrc.game_data.games_metadata.as_ref()).vec_game_metadata;
    let mut last_name = unwrap!(gmd.first()).name.to_string();
    for x in gmd.iter().rev() {
        if rrc.game_data.game_name.as_str() == last_name.as_str() {
            rrc.game_data.game_name = x.name.to_string();
            vdom.schedule_render();
            break;
        }
        last_name = x.name.to_string();
    }
    fetchgameconfigmod::async_fetch_game_config_request(rrc, vdom);
}

/// get value form input html element by id
fn get_input_value(id: &str) -> String {
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document(), "document");
    //logmod::debug_write(&format!("before get_element_by_id: {}", id));
    let input_el = unwrap!(document.get_element_by_id(id));
    //logmod::debug_write("before dyn_into");
    let input_html_element = unwrap!(input_el.dyn_into::<web_sys::HtmlInputElement>(), "dyn_into");
    //return
    input_html_element.value()
}

/// fn open new local page with # window.location.set_hash
pub fn open_new_local_page(hash: &str) {
    let window = unwrap!(web_sys::window());
    let _x = window.location().set_hash(hash);
}

/// send a message to the first player
/// return the text for html template replace
pub fn group_id_joined(rrc: &RootRenderingComponent) -> String {
    let group_id = unwrap!(rrc.game_data.players.get(0)).ws_uid;
    let group_id = format!("{}", group_id);
    group_id
}
