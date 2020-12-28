// statusgameovermod.rs
//! code flow from this status

#![allow(clippy::panic)]

// region: use
use crate::*;

use unwrap::unwrap;
use dodrio::{RenderContext, Node};
use crate::htmltemplatemod::HtmlTemplating;
// endregion

/// play again
pub fn div_game_over<'a>(rrc: &RootRenderingComponent, cx: &mut RenderContext<'a>) -> Node<'a> {
    // game over
    // only the leader of the group player can choose Play again?
    // other players are already joined to the group
    let template_name = if rrc.game_data.my_player_number == 1 {
        "play_again"
    } else {
        "game_over"
    };
    let html_template = rrc.web_data.get_sub_template(template_name);
    unwrap!(rrc.render_template(cx, &html_template, htmltemplatemod::HtmlOrSvg::Html))
}

/// on msg game over
pub fn on_msg_game_over(rrc: &mut RootRenderingComponent) {
    // The game is over.
    rrc.game_data.game_status = GameStatus::StatusGameOver;
}

/// on msg play again
pub fn on_msg_play_again(rrc: &mut RootRenderingComponent) {
    // The first players can choose Play again and send to others.
    rrc.game_data.game_status = GameStatus::StatusJoined;
    rrc.game_data.reset_for_play_again();
    htmltemplateimplmod::open_new_local_page("#p04");
}
