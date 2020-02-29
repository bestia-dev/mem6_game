// divplayeractionsmod.rs
//! renders the div to inform player what to do next
//! and get a click action from the user

//region: use
use crate::*;

use mem6_common::*;
use crate::htmltemplatemod::HtmlTemplating;
use dodrio::{RenderContext, Node};
use unwrap::unwrap;
//endregion

/// render html element to inform player what to do and get a click action from user
pub fn div_player_actions_from_game_status<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
) -> Node<'a> {
    // if rrc.game_data.is_status_start_page() {
    /*
        && (rrc.web_data.is_reconnect || rrc.web_data.ws.ready_state() != 1)
    {
        // ready_state: 0	CONNECTING, 1	OPEN, 2	CLOSING, 3	CLOSED
        statusreconnectmod::div_reconnect(rrc, bump)
    */
    if let GameStatus::Status1stCard = rrc.game_data.game_status {
        status1stcardmod::div_on_1st_card(rrc, cx)
    } else if let GameStatus::Status2ndCard = rrc.game_data.game_status {
        status2ndcardmod::div_click_2nd_card(rrc, cx)
    } else if let GameStatus::StatusGameOver = rrc.game_data.game_status {
        statusgameovermod::div_game_over(rrc, cx)
    } else if let GameStatus::StatusWaitingAckMsg = rrc.game_data.game_status {
        statuswaitingackmsgmod::div_waiting_ack_msg(rrc, cx)
    } else {
        div_unpredicted(rrc, cx)
    }
}

/// render unpredicted
fn div_unpredicted<'a>(rrc: &RootRenderingComponent, cx: &mut RenderContext<'a>) -> Node<'a> {
    // unpredictable situation
    let html_template = r#"<h2>
    gamestatus: <!--t=game_status--> one, player<!--t=my_player_number--> Nick
    </h2>"#;
    // return
    unwrap!(rrc.render_template(cx, html_template, htmltemplatemod::HtmlOrSvg::Html))
}
