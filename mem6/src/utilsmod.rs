//utilsmod.rs
//! small generic helper functions

use crate::*;
use mem6_common::*;
use unwrap::unwrap;

/// return window object
pub fn window() -> web_sys::Window {
    unwrap!(web_sys::window())
}

/// add the first player as group_id so the msg can be sent to him
pub fn push_first_player_as_group_id(rrc: &mut RootRenderingComponent, group_id: &str) {
    let group_id = if group_id.is_empty() { "0" } else { group_id };
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
