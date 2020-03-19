// statustaketurnmod.rs
//! code flow from this status

// region: use
use crate::*;

use unwrap::unwrap;
// endregion

/// on click
pub fn on_click_take_turn(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    // websysmod::debug_write(&format!("on_click_take_turn {}", ""));

    let msg_id = ackmsgmod::prepare_for_ack_msg_waiting(rrc, vdom);

    let msg = websocketmod::WsMessageForReceivers {
        msg_sender_ws_uid: rrc.web_data.my_ws_uid,
        msg_receivers_json: rrc.web_data.msg_receivers_json.to_string(),
        msg_data: gamedatamod::WsMessageGameData::MsgTakeTurn { msg_id },
    };
    ackmsgmod::send_msg_and_write_in_queue(rrc, &msg, msg_id);

    // Here I wait for on_MsgAck from
    // every player before call update_take_turn(rrc);
}

/// on msg
pub fn on_msg_take_turn(rrc: &mut RootRenderingComponent, msg_sender_ws_uid: usize, msg_id: usize) {
    ackmsgmod::send_ack(
        rrc,
        msg_sender_ws_uid,
        msg_id,
        gamedatamod::MsgAckKind::MsgTakeTurn,
    );
    update_on_take_turn(rrc);
}

/// on msg ack
#[allow(clippy::needless_pass_by_value)]
pub fn on_msg_ack_take_turn(rrc: &mut RootRenderingComponent, player_ws_uid: usize, msg_id: usize) {
    if ackmsgmod::remove_ack_msg_from_queue(rrc, player_ws_uid, msg_id) {
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
