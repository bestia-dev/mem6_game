//ackmsgmod.rs
//! acknowledge msg

//region: use
use crate::*;

use mem6_common::*;

use unwrap::unwrap;
//endregion

/// remove ack msg from queue - return true if there are no more msgs
#[allow(clippy::needless_pass_by_value)]
pub fn remove_ack_msg_from_queue(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
) -> bool {
    //remove the waiting msg from the queue
    //I use the opposite method "retain" because there is not a method "remove"
    rrc.game_data
        .msgs_waiting_ack
        .retain(|x| !(x.player_ws_uid == player_ws_uid && x.msg_id == msg_id));

    //if there is no more items with this msg_id, then proceed
    let mut has_msg_id = false;
    for x in &rrc.game_data.msgs_waiting_ack {
        if x.msg_id == msg_id {
            has_msg_id = true;
            break;
        }
    }
    //return
    !has_msg_id
}

/// prepare for ack msg waiting - return random msg_id
pub fn prepare_for_ack_msg_waiting(
    rrc: &mut RootRenderingComponent,
    vdom: &dodrio::VdomWeak,
) -> usize {
    let msg_id = windowmod::get_random(1, 0xFFFF_FFFF);
    rrc.game_data.game_status = GameStatus::StatusWaitingAckMsg;
    vdom.schedule_render();
    //return
    msg_id
}

/// send msg and write in queue
pub fn send_msg_and_write_in_queue(
    rrc: &mut RootRenderingComponent,
    msg: &WsMessage,
    msg_id: usize,
) {
    //write the msgs in the queue
    for player in &rrc.game_data.players {
        if player.ws_uid != rrc.game_data.my_ws_uid {
            let msg_for_loop = msg.clone();
            rrc.game_data
                .msgs_waiting_ack
                .push(gamedatamod::MsgInQueue {
                    player_ws_uid: player.ws_uid,
                    msg_id,
                    msg: msg_for_loop,
                });
        }
    }
    websocketcommunicationmod::ws_send_msg(&rrc.game_data.ws, msg);
}

/// send ack
pub fn send_ack(
    rrc: &mut RootRenderingComponent,
    msg_sender_ws_uid: usize,
    msg_id: usize,
    msg_ack_kind: MsgAckKind,
) {
    //logmod::debug_write(&format!("send_ack players: {:?}", rrc.game_data.players));
    //send back the ACK msg to the sender
    websocketcommunicationmod::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::MsgAck {
            my_ws_uid: rrc.game_data.my_ws_uid,
            players_ws_uid: unwrap!(serde_json::to_string(&vec![msg_sender_ws_uid])),
            msg_id,
            msg_ack_kind,
        },
    );
}
