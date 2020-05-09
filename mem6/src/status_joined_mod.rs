// status_joined_mod.rs
//! code flow for this status

#![allow(clippy::panic)]

// region: use
use crate::*;

// use unwrap::unwrap;

// endregion

/// group_id is the ws_uid of the first player
pub fn on_load_joined(rrc: &mut RootRenderingComponent) {
    rrc.game_data.game_status = GameStatus::StatusJoined;
    websysmod::debug_write(&format!(
        "StatusJoined send {}",
        rrc.web_data.msg_receivers_json
    ));

    rrc.web_data
        .send_ws_msg_from_web_data(&websocket_boiler_mod::WsMessageForReceivers {
            msg_sender_ws_uid: rrc.web_data.my_ws_uid,
            msg_receivers_json: rrc.web_data.msg_receivers_json.to_string(),
            msg_data: game_data_mod::WsMessageGameData::MsgJoin {
                my_nickname: rrc.game_data.my_nickname.clone(),
            },
        });
}

/// msg joined
pub fn on_msg_joined(rrc: &mut RootRenderingComponent, his_ws_uid: usize, his_nickname: String) {
    // websysmod::debug_write(&format!("on_msg_joined {}",his_ws_uid));
    if rrc.game_data.my_player_number == 1 {
        // push if not exists
        let mut ws_uid_exists = false;
        for x in &rrc.game_data.players {
            if x.ws_uid == his_ws_uid {
                ws_uid_exists = true;
                break;
            }
        }
        if !ws_uid_exists {
            rrc.game_data.players.push(Player {
                ws_uid: his_ws_uid,
                nickname: his_nickname,
                points: 0,
            });
            rrc.web_data.msg_receivers_json = rrc.game_data.prepare_json_msg_receivers();
        }
    }
}
