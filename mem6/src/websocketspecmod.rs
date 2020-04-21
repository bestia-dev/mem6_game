// websocketspecmod.rs
//! Specific code for this project, that is not Boilerplate code.  

#![allow(clippy::panic)]

// region: use
use crate::*;
use rust_wasm_webrtc::webrtcmod::{WebRtcTrait};

//use unwrap::unwrap;
use dodrio::{VdomWeak};
// endregion

pub fn match_msg_and_call_function( vdom: VdomWeak,rrc:&mut RootRenderingComponent,msg: websocketboilermod::WsMessageForReceivers) {
    match msg.msg_data {
        WsMessageGameData::MsgJoin {
            my_nickname,
        } => {
            statusjoinedmod::on_msg_joined(rrc, msg.msg_sender_ws_uid, my_nickname);
            vdom.schedule_render();
        }
        WsMessageGameData::MsgStartGame {
            
            card_grid_data,
            game_config,
            players,
            game_name,
            player_turn,
        } => {
            statusgamedatainitmod::on_msg_start_game(
                rrc,
                &card_grid_data,
                &game_config,
                &players,
                &game_name,
                player_turn,
            );
            htmltemplateimplmod::open_new_local_page("#p11");
            vdom.schedule_render();
        }
        WsMessageGameData::MsgClick1stCard {
            
            card_index_of_1st_click,
            msg_id,
        } => {
            status1stcardmod::on_msg_click_1st_card(
                rrc,
                vdom.clone(),
                msg.msg_sender_ws_uid,
                card_index_of_1st_click,
                msg_id,
            );
            vdom.schedule_render();
        }
        WsMessageGameData::MsgClick2ndCard {
            card_index_of_2nd_click,
            is_point,
            msg_id,
        } => {
            status2ndcardmod::on_msg_click_2nd_card(
                rrc,
                msg.msg_sender_ws_uid,
                card_index_of_2nd_click,
                is_point,
                msg_id,
            );
            vdom.schedule_render();
        }
        WsMessageGameData::MsgDrinkEnd {
            
        } => {
            statusdrinkmod::on_msg_drink_end(rrc, msg.msg_sender_ws_uid, vdom.clone());
            vdom.schedule_render();
        }
        WsMessageGameData::MsgTakeTurn {
            
            
            msg_id,
        } => {
            statustaketurnmod::on_msg_take_turn(rrc, msg.msg_sender_ws_uid, msg_id);
            vdom.schedule_render();
        }
        WsMessageGameData::MsgGameOver {
            
        } => {
            statusgameovermod::on_msg_game_over(rrc);
            vdom.schedule_render();
        }
        WsMessageGameData::MsgPlayAgain {
                                                    } => {
            statusgameovermod::on_msg_play_again(rrc);
        }
        WsMessageGameData::MsgSoundsAndLabels {
            sounds_and_labels,
        
            
        } => {
            rrc.game_data.sounds_and_labels=sounds_and_labels;
            vdom.schedule_render();
        }
        WsMessageGameData::MsgAck {
            
            msg_id,
            msg_ack_kind,
        } => {
            match msg_ack_kind {
                MsgAckKind::MsgTakeTurn => {
                    statustaketurnmod::on_msg_ack_take_turn(
                        rrc, msg.msg_sender_ws_uid, msg_id,
                    );
                }
                MsgAckKind::MsgClick1stCard => {
                    status1stcardmod::on_msg_ack_click_1st_card(
                        rrc, msg.msg_sender_ws_uid, msg_id,
                    );
                }
                MsgAckKind::MsgClick2ndCard => {
                    status2ndcardmod::on_msg_ack_player_click2nd_card(
                        rrc, msg.msg_sender_ws_uid, msg_id, vdom.clone(),
                    );
                }
            }
            vdom.schedule_render();
        }
        WsMessageGameData::MsgAskPlayer1ForResync {
            
        } => {
            statusreconnectmod::send_msg_for_resync(rrc);
            vdom.schedule_render();
        }
        WsMessageGameData::MsgAllGameData {
            
            players,
            card_grid_data,
            card_index_of_1st_click,
            card_index_of_2nd_click,
            player_turn,
            game_status,
        } => {
            statusreconnectmod::on_msg_all_game_data(
                rrc,
                players,
                card_grid_data,
                card_index_of_1st_click,
                card_index_of_2nd_click,
                player_turn,
                game_status,
            );
            vdom.schedule_render();
        }
        WsMessageGameData::MsgWebrtcOffer{
            sdp
        }=>{
            rrc.web_data.web_rtc_data.web_rtc_receive_offer(vdom.clone(),sdp, msg.msg_sender_ws_uid);
        }
        WsMessageGameData::MsgWebrtcAnswer{
            sdp
        }=>{
            rrc.web_data.web_rtc_data.web_rtc_receive_answer(vdom.clone(),sdp);
        }
        WsMessageGameData::MsgWebrtcIceCandidate{
            sdp
        }=>{
            rrc.web_data.web_rtc_data.web_rtc_receive_ice_candidate(vdom.clone(),sdp);
        }
    }
}
    