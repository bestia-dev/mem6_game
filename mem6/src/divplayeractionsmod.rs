// divplayeractionsmod.rs
//! renders the div to inform player what to do next
//! and get a click action from the user

// region: use
use crate::*;

use crate::htmltemplatemod::HtmlTemplating;
use dodrio::{RenderContext, Node};
use unwrap::unwrap;
// endregion

/// render html element to inform player what to do and get a click action from user
pub fn div_player_actions_from_game_status<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
) -> Node<'a> {
    match rrc.game_data.game_status {
        GameStatus::Status1stCard => status1stcardmod::div_on_1st_card(rrc, cx),
        GameStatus::Status2ndCard => status2ndcardmod::div_click_2nd_card(rrc, cx),
        GameStatus::StatusGameOver => statusgameovermod::div_game_over(rrc, cx),
        GameStatus::StatusWaitingAckMsg => statuswaitingackmsgmod::div_waiting_ack_msg(rrc, cx),
        _ => div_unpredicted(rrc, cx),
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
