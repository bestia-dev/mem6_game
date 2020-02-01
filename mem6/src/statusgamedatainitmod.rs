// statusgamedatainitmod.rs
//! code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::*;
use mem6_common::*;

use unwrap::unwrap;
//endregion

///prepares the game data
pub fn on_click_start_game(rrc: &mut RootRenderingComponent) {
    rrc.game_data.prepare_random_data();
    rrc.game_data.game_status = GameStatus::Status1stCard;
    rrc.game_data.player_turn = 1;

    websocketcommunicationmod::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::MsgStartGame {
            my_ws_uid: rrc.game_data.my_ws_uid,
            players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
            players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
            card_grid_data: unwrap!(serde_json::to_string(&rrc.game_data.card_grid_data)),
            game_config: unwrap!(serde_json::to_string(&rrc.game_data.game_config)),
            game_name: rrc.game_data.game_name.to_string(),
        },
    );
}

///on game data init
pub fn on_msg_start_game(
    rrc: &mut RootRenderingComponent,
    card_grid_data: &str,
    game_config: &str,
    players: &str,
    game_name: &str,
) {
    //logmod::debug_write(&format!("on_msg_start_game {}", players));
    rrc.game_data.game_status = GameStatus::Status1stCard;
    rrc.game_data.player_turn = 1;
    rrc.game_data.game_name = game_name.to_string();

    rrc.game_data.game_config = unwrap!(
        serde_json::from_str(game_config),
        "error serde_json::from_str(game_config)"
    );

    rrc.game_data.card_grid_data = unwrap!(
        serde_json::from_str(card_grid_data),
        "error serde_json::from_str(card_grid_data)"
    );

    //async fetch all imgs and put them in service worker cache
    fetchallimgsforcachemod::fetch_all_img_for_cache_request(rrc);

    rrc.game_data.players = unwrap!(
        serde_json::from_str(players),
        "error serde_json::from_str(players)"
    );

    rrc.game_data.players_ws_uid = gamedatamod::prepare_players_ws_uid(&rrc.game_data.players);

    //find my player number
    for index in 0..rrc.game_data.players.len() {
        if unwrap!(
            rrc.game_data.players.get_mut(index),
            "rrc.game_data.players.get_mut(index)"
        )
        .ws_uid
            == rrc.game_data.my_ws_uid
        {
            rrc.game_data.my_player_number = unwrap!(index.checked_add(1));
        }
    }
}
