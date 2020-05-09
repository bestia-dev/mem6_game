// ack_msg_mod.rs
//! acknowledge msg

// region: use
use crate::*;

use unwrap::unwrap;
use dodrio::VdomWeak;
// endregion

/// remove ack msg from queue - return true if there are no more msgs
#[allow(clippy::needless_pass_by_value)]
pub fn remove_ack_msg_from_queue(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
) -> bool {
    // remove the waiting msg from the queue
    // I use the opposite method "retain" because there is not a method "remove"
    rrc.web_data
        .msgs_waiting_ack
        .retain(|x| !(x.player_ws_uid == player_ws_uid && x.msg_id == msg_id));

    // if there is no more items with this msg_id, then proceed
    let mut has_msg_id = false;
    for x in &rrc.web_data.msgs_waiting_ack {
        if x.msg_id == msg_id {
            has_msg_id = true;
            break;
        }
    }
    // return
    !has_msg_id
}

/// prepare for ack msg waiting - return random msg_id
pub fn prepare_for_ack_msg_waiting(rrc: &mut RootRenderingComponent, vdom: VdomWeak) -> usize {
    let msg_id = websysmod::get_random(1, 0xFFFF_FFFF);
    rrc.game_data.game_status = GameStatus::StatusWaitingAckMsg;
    vdom.schedule_render();
    // return
    msg_id
}

/// send msg and write in queue
pub fn send_msg_and_write_in_queue(
    rrc: &mut RootRenderingComponent,
    msg: &websocket_boiler_mod::WsMessageForReceivers,
    msg_id: usize,
) {
    // write the msgs in the queue
    for player in &rrc.game_data.players {
        if player.ws_uid != rrc.web_data.my_ws_uid {
            let msg_for_loop = msg.clone();
            rrc.web_data.msgs_waiting_ack.push(web_data_mod::MsgInQueue {
                player_ws_uid: player.ws_uid,
                msg_id,
                msg: msg_for_loop,
            });
        }
    }
    rrc.web_data.send_ws_msg_from_web_data(msg);
}

/// send ack
pub fn send_ack(
    rrc: &mut RootRenderingComponent,
    msg_sender_ws_uid: usize,
    msg_id: usize,
    msg_ack_kind: game_data_mod::MsgAckKind,
) {
    // websysmod::debug_write(&format!("send_ack players: {:?}", rrc.game_data.players));
    // send back the ACK msg to the sender
    rrc.web_data
        .send_ws_msg_from_web_data(&websocket_boiler_mod::WsMessageForReceivers {
            msg_sender_ws_uid: rrc.web_data.my_ws_uid,
            msg_receivers_json: unwrap!(serde_json::to_string(&vec![msg_sender_ws_uid])),
            msg_data: game_data_mod::WsMessageGameData::MsgAck {
                msg_id,
                msg_ack_kind,
            },
        });
}
