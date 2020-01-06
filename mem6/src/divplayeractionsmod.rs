// divplayeractionsmod.rs
//! renders the div to inform player what to do next
//! and get a click action from the user

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::statusgameovermod;
use crate::status1stcardmod;
use crate::status2ndcardmod;
use crate::statustaketurnbeginmod;
use crate::statusstartpagemod;
use crate::statusinvitedmod;
use crate::statusinvitingmod;
use crate::statuswaitingackmsgmod;
//use crate::websocketreconnectmod;

use mem6_common::{GameStatus};

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///render html element to inform player what to do and get a click action from user
pub fn div_player_actions_from_game_status<'a>(
    rrc: &RootRenderingComponent,
    bump: &'a Bump,
) -> Node<'a> {
    //if rrc.game_data.is_status_start_page() {
    /*
        && (rrc.game_data.is_reconnect || rrc.game_data.ws.ready_state() != 1)
    {
        //ready_state: 0	CONNECTING, 1	OPEN, 2	CLOSING, 3	CLOSED
        websocketreconnectmod::div_reconnect(rrc, bump)
    */
    if let GameStatus::StatusStartPage = rrc.game_data.game_status {
        statusstartpagemod::div_start_page(rrc, bump)
    } else if let GameStatus::StatusInvited = rrc.game_data.game_status {
        statusinvitedmod::div_invited(rrc, bump)
    } else if let GameStatus::StatusInviting = rrc.game_data.game_status {
        statusinvitingmod::div_inviting(rrc, bump)
    } else if let GameStatus::StatusAccepted = rrc.game_data.game_status {
        statusinvitedmod::div_invite_accepted(rrc, bump)
    } else if let GameStatus::Status1stCard = rrc.game_data.game_status {
        status1stcardmod::div_on_1st_card(rrc, bump)
    } else if let GameStatus::Status2ndCard = rrc.game_data.game_status {
        status2ndcardmod::div_click_2nd_card(rrc, bump)
    } else if let GameStatus::StatusTakeTurnBegin = rrc.game_data.game_status {
        statustaketurnbeginmod::div_take_turn_begin(rrc, bump)
    } else if let GameStatus::StatusGameOver = rrc.game_data.game_status {
        statusgameovermod::div_game_over(rrc, bump)
    } else if let GameStatus::StatusWaitingAckMsg = rrc.game_data.game_status {
        statuswaitingackmsgmod::div_waiting_ack_msg(rrc, bump)
    } else {
        div_unpredicted(rrc, bump)
    }
}

///render unpredicted
fn div_unpredicted<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    //unpredictable situation
    //return
    dodrio!(bump,
    <h2  >
        {vec![text(bumpalo::format!(in bump, "gamestatus: {} player {}", rrc.game_data.game_status.as_ref(),rrc.game_data.my_player_number).into_bump_str())]}
    </h2>
    )
}
