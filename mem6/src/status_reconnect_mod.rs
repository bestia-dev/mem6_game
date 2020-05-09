// status_reconnect_mod.rs
//! reconnection for websocket must be part of the application.

// Websocket has a lot of problems with maintaining a stable connection.
// When a player is out of sync with others it is probably because
// of a websocket connection problem.
// The button Resync first connects to the ws server and sends a msg to other players.
// All other players send him the complete data. He uses only the data from the first msg
// he receives and ignore all others.

// macro dodrio ! now has warning about a panic?!?
#![allow(clippy::panic)]

// region: use
use crate::*;

use unwrap::unwrap;

// use dodrio::{};
// endregion
/*
/// render reconnect
pub fn div_reconnect<'a>(_rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    dodrio !(bump,
    <div>
      <h4>
            {vec![text(bumpalo::format!(in bump,
            "Click on Resync if there are problems with receiving msgs over the network:{}", "")
            .into_bump_str(),)]}
        </h4>
        <div class="div_clickable" onclick={
            move |root, vdom, _event| {
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            // the old ws and closures are now a memory leak, but small
            let href = rrc.web_data.href.clone();
            // usize is Copy(), so I don't need clone()
            let my_ws_uid = rrc.web_data.my_ws_uid;
            websysmod::debug_write(&format!(
                "href {}  my_ws_uid {}",
                href,
                my_ws_uid,
            ));
            // websysmod::debug_write(&"before reconnect");
            // first disconnect if is possible, than reconnect
            let _x = rrc.web_data.websocket_data.ws.close();

            let msg_receivers_json = rrc.web_data.msg_receivers_json.clone();
            let ws = websocketmod::setup_ws_connection(href, my_ws_uid,msg_receivers_json);
            websocket_boiler_mod::setup_all_ws_events(&ws,vdom.clone());

            rrc.web_data.ws=ws;
            // websysmod::debug_write(&"before game_data.web_data.is_reconnect = false and schedule_render");
            rrc.web_data.is_reconnect = false;
            vdom.schedule_render();
        }}>
            <h2 class="h2_user_can_click">
                {vec![text(
                // StatusReconnect?
                bumpalo::format!(in bump, "Resync{}", "").into_bump_str(),
                )]}
            </h2>
        </div>
    </div>
    )
}
*/
/// send all data to resync game_data
pub fn send_msg_for_resync(rrc: &RootRenderingComponent) {
    websysmod::debug_write("send_msg_for_resync MsgAllGameData");
    rrc.web_data
        .send_ws_msg_from_web_data(&websocket_boiler_mod::WsMessageForReceivers {
            msg_sender_ws_uid: rrc.web_data.my_ws_uid,
            /// only the players that resync
            msg_receivers_json: rrc.web_data.msg_receivers_json.clone(),
            msg_data: game_data_mod::WsMessageGameData::MsgAllGameData {
                /// json of vector of players with nicknames and order data
                players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
                /// vector of cards status
                card_grid_data: unwrap!(serde_json::to_string(&rrc.game_data.card_grid_data)),
                card_index_of_1st_click: rrc.game_data.card_index_of_1st_click,
                card_index_of_2nd_click: rrc.game_data.card_index_of_2nd_click,
                /// whose turn is now:  player 1,2,3,...
                player_turn: rrc.game_data.player_turn,
                /// game status, strum Display converts into String
                game_status: format!("{}", rrc.game_data.game_status),
            },
        });
}

/// after reconnect receive all the data from other player
#[allow(clippy::needless_pass_by_value)]
pub fn on_msg_all_game_data(
    rrc: &mut RootRenderingComponent,
    players: String,
    card_grid_data: String,
    card_index_of_1st_click: usize,
    card_index_of_2nd_click: usize,
    // whose turn is now:  player 1,2,3,...
    player_turn: usize,
    game_status: String,
) {
    websysmod::debug_write("on_msg_all_game_data");
    //strum EnumString adds the from_str function
    use std::str::FromStr;
    let game_status = GameStatus::from_str(&game_status).unwrap();

    // only the first message is processed
    // if rrc.game_data.web_data.is_reconnect {
    rrc.web_data.is_reconnect = false;
    rrc.game_data.players = unwrap!(serde_json::from_str(&players));
    rrc.game_data.card_grid_data = unwrap!(serde_json::from_str(&card_grid_data));
    rrc.game_data.card_index_of_1st_click = card_index_of_1st_click;
    rrc.game_data.card_index_of_2nd_click = card_index_of_2nd_click;
    rrc.game_data.player_turn = player_turn;
    rrc.game_data.game_status = game_status;
    rrc.web_data.msgs_waiting_ack.retain(|_x| false);
    // }
}
