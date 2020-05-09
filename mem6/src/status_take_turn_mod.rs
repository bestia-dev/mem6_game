// status_take_turn_mod.rs
//! code flow from this status

// region: use
use crate::*;

use unwrap::unwrap;
use dodrio::VdomWeak;
// endregion

/// on click
pub fn on_click_take_turn(rrc: &mut RootRenderingComponent, vdom: VdomWeak) {
    // websysmod::debug_write(&format!("on_click_take_turn {}", ""));

    let msg_id = ack_msg_mod::prepare_for_ack_msg_waiting(rrc, vdom.clone());

    let msg = websocket_boiler_mod::WsMessageForReceivers {
        msg_sender_ws_uid: rrc.web_data.my_ws_uid,
        msg_receivers_json: rrc.web_data.msg_receivers_json.to_string(),
        msg_data: game_data_mod::WsMessageGameData::MsgTakeTurn { msg_id },
    };
    ack_msg_mod::send_msg_and_write_in_queue(rrc, &msg, msg_id);

    // Here I wait for on_MsgAck from
    // every player before call update_take_turn(rrc);
}

/// on msg
pub fn on_msg_take_turn(rrc: &mut RootRenderingComponent, msg_sender_ws_uid: usize, msg_id: usize) {
    ack_msg_mod::send_ack(
        rrc,
        msg_sender_ws_uid,
        msg_id,
        game_data_mod::MsgAckKind::MsgTakeTurn,
    );
    update_on_take_turn(rrc);
}

/// on msg ack
#[allow(clippy::needless_pass_by_value)]
pub fn on_msg_ack_take_turn(rrc: &mut RootRenderingComponent, player_ws_uid: usize, msg_id: usize) {
    if ack_msg_mod::remove_ack_msg_from_queue(rrc, player_ws_uid, msg_id) {
        update_on_take_turn(rrc);
    }
    // TODO: timer if after 3 seconds the ack is not received resend the msg
    // do this 3 times and then hard error
}

/// update game data
pub fn update_on_take_turn(rrc: &mut RootRenderingComponent) {
    rrc.game_data.player_turn = if rrc.game_data.player_turn < rrc.game_data.players.len() {
        unwrap!(rrc.game_data.player_turn.checked_add(1))
    } else {
        1
    };

    rrc.game_data.game_status = GameStatus::Status1stCard;
}

// there is no special div render, because it jumps to StatusBefore1stClick
