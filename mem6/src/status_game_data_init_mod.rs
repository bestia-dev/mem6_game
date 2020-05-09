// status_game_data_init_mod.rs
//! code flow from this status

// region: use
use crate::*;

use unwrap::unwrap;
// endregion

/// prepares the game data
pub fn on_click_start_game(rrc: &mut RootRenderingComponent) {
    rrc.game_data.prepare_random_data();
    rrc.game_data.game_status = GameStatus::Status1stCard;
    // random start player_turn. So is not always the first player to start
    // gen_range is lower inclusive, upper exclusive
    rrc.game_data.player_turn =
        websysmod::get_random(1, unwrap!(rrc.game_data.players.len().checked_add(1)));

    rrc.web_data
        .send_ws_msg_from_web_data(&websocket_boiler_mod::WsMessageForReceivers {
            msg_sender_ws_uid: rrc.web_data.my_ws_uid,
            msg_receivers_json: rrc.web_data.msg_receivers_json.to_string(),
            msg_data: game_data_mod::WsMessageGameData::MsgStartGame {
                players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
                card_grid_data: unwrap!(serde_json::to_string(&rrc.game_data.card_grid_data)),
                game_config: unwrap!(serde_json::to_string(&rrc.game_data.game_config)),
                game_name: rrc.game_data.game_name.to_string(),
                player_turn: rrc.game_data.player_turn,
            },
        });
}

/// on game data init
#[allow(clippy::indexing_slicing)]
pub fn on_msg_start_game(
    rrc: &mut RootRenderingComponent,
    card_grid_data: &str,
    game_config: &str,
    players: &str,
    game_name: &str,
    player_turn: usize,
) {
    // websysmod::debug_write(&format!("on_msg_start_game {}", players));
    rrc.game_data.game_status = GameStatus::Status1stCard;
    rrc.game_data.player_turn = player_turn;
    rrc.game_data.game_name = game_name.to_string();

    rrc.game_data.game_config = unwrap!(
        serde_json::from_str(game_config),
        "error serde_json::from_str(game_config)"
    );

    rrc.game_data.card_grid_data = unwrap!(
        serde_json::from_str(card_grid_data),
        "error serde_json::from_str(card_grid_data)"
    );

    // async fetch all imgs and put them in service worker cache
    fetch_mod::fetch_all_img_for_cache_request(rrc);

    rrc.game_data.players = unwrap!(
        serde_json::from_str(players),
        "error serde_json::from_str(players)"
    );

    rrc.web_data.msg_receivers_json = rrc.game_data.prepare_json_msg_receivers();

    // find my player number
    for index in 0..rrc.game_data.players.len() {
        if rrc.game_data.players[index].ws_uid == rrc.web_data.my_ws_uid {
            rrc.game_data.my_player_number = unwrap!(index.checked_add(1));
        }
    }
}
