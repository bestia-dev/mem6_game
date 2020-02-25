// divplayeractionsmod.rs
//! renders the div to inform player what to do next
//! and get a click action from the user

//region: use
use crate::*;

use mem6_common::*;

use dodrio::{
    builder::{ElementBuilder, text},
    bumpalo::{self, Bump},
    Node,
};
//endregion

/// render html element to inform player what to do and get a click action from user
pub fn div_player_actions_from_game_status<'a>(
    rrc: &RootRenderingComponent,
    bump: &'a Bump,
) -> Node<'a> {
    // if rrc.game_data.is_status_start_page() {
    /*
        && (rrc.web_data.is_reconnect || rrc.web_data.ws.ready_state() != 1)
    {
        // ready_state: 0	CONNECTING, 1	OPEN, 2	CLOSING, 3	CLOSED
        statusreconnectmod::div_reconnect(rrc, bump)
    */
    if let GameStatus::Status1stCard = rrc.game_data.game_status {
        status1stcardmod::div_on_1st_card(rrc, bump)
    } else if let GameStatus::Status2ndCard = rrc.game_data.game_status {
        status2ndcardmod::div_click_2nd_card(rrc, bump)
    } else if let GameStatus::StatusGameOver = rrc.game_data.game_status {
        statusgameovermod::div_game_over(rrc, bump)
    } else if let GameStatus::StatusWaitingAckMsg = rrc.game_data.game_status {
        statuswaitingackmsgmod::div_waiting_ack_msg(rrc, bump)
    } else {
        div_unpredicted(rrc, bump)
    }
}

/// render unpredicted
fn div_unpredicted<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    // unpredictable situation
    // return
    ElementBuilder::new(bump, "h2")
        .children([text(
            bumpalo::format!(in bump, "gamestatus: {} player {}", 
        rrc.game_data.game_status.as_ref(),rrc.game_data.my_player_number)
            .into_bump_str(),
        )])
        .finish()
}
