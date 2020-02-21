// websocketreconnectmod.rs
//! reconnection for websocket must be part of the application.

// Websocket has a lot of problems with maintaining a stable connection.
// When a player is out of sync with others it is probably because
// of a websocket connection problem.
// The button Resync first connects to the ws server and sends a msg to other players.
// All other players send him the complete data. He uses only the data from the first msg
// he receives and ignore all others.

//macro dodrio! now has warning about a panic?!?
#![allow(clippy::panic)]

//region: use
use crate::*;

use mem6_common::*;

use unwrap::unwrap;
use dodrio::{
    builder::text,
    bumpalo::{self, Bump},
    Node,
};
use typed_html::dodrio;
//endregion

/// render reconnect
pub fn div_reconnect<'a>(_rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    dodrio!(bump,
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
            let href = rrc.game_data.href.clone();
            // usize is Copy(), so I don't need clone()
            let my_ws_uid = rrc.game_data.my_ws_uid;
            logmod::debug_write(&format!(
                "href {}  my_ws_uid {}",
                href,
                my_ws_uid,
            ));
            // logmod::debug_write(&"before reconnect");
            // first disconnect if is possible, than recconect
            let _x = rrc.game_data.ws.close();

            let msg_receivers = rrc.game_data.msg_receivers.clone();
            let ws = websocketcommunicationmod::setup_ws_connection(href, my_ws_uid,msg_receivers);
            websocketcommunicationmod::setup_all_ws_events(&ws,vdom.clone());

            rrc.game_data.ws=ws;
            // logmod::debug_write(&"before game_data.is_reconnect = false and schedule_render");
            rrc.game_data.is_reconnect = false;
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

/// send all data to resync gamedata
pub fn send_msg_for_resync(rrc: &RootRenderingComponent) {
    logmod::debug_write("send_msg_for_resync MsgAllGameData");
    websocketcommunicationmod::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::MsgAllGameData {
            my_ws_uid: rrc.game_data.my_ws_uid,
            // / only the players that resync
            msg_receivers: rrc.game_data.msg_receivers.clone(),
            // / json of vector of players with nicknames and order data
            players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
            // / vector of cards status
            card_grid_data: unwrap!(serde_json::to_string(&rrc.game_data.card_grid_data)),
            card_index_of_first_click: rrc.game_data.card_index_of_first_click,
            card_index_of_second_click: rrc.game_data.card_index_of_second_click,
            // / whose turn is now:  player 1,2,3,...
            player_turn: rrc.game_data.player_turn,
            // / game status
            game_status: rrc.game_data.game_status.clone(),
        },
    );
}

/// after reconnect receive all the data from other player
#[allow(clippy::needless_pass_by_value)]
pub fn on_msg_all_game_data(
    rrc: &mut RootRenderingComponent,
    players: String,
    card_grid_data: String,
    card_index_of_first_click: usize,
    card_index_of_second_click: usize,
    // whose turn is now:  player 1,2,3,...
    player_turn: usize,
    game_status: GameStatus,
) {
    logmod::debug_write("on_msg_all_game_data");
    // only the first message is processed
    // if rrc.game_data.is_reconnect {
    rrc.game_data.is_reconnect = false;
    rrc.game_data.players = unwrap!(serde_json::from_str(&players));
    rrc.game_data.card_grid_data = unwrap!(serde_json::from_str(&card_grid_data));
    rrc.game_data.card_index_of_first_click = card_index_of_first_click;
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;
    rrc.game_data.player_turn = player_turn;
    rrc.game_data.game_status = game_status;
    rrc.game_data.msgs_waiting_ack.retain(|_x| false);
    // }
}
