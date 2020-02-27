// statusgameovermod.rs
//! code flow from this status

#![allow(clippy::panic)]

//region: use
use crate::*;
use mem6_common::*;

//use unwrap::unwrap;
use dodrio::{
    builder::{ElementBuilder, text},
    bumpalo::{self, Bump},
    Node,
};
//endregion

/// play again
pub fn div_game_over<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    // game over
    // only the leader of the group player can choose Play again?
    // other players are already joined to the group
    if rrc.game_data.my_player_number == 1 {
        ElementBuilder::new(bump, "div")
            .on("click", move |root, _vdom, _event| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                websocketmod::ws_send_msg(
                    &rrc.web_data.ws,
                    &WsMessage::MsgPlayAgain {
                        my_ws_uid: rrc.web_data.my_ws_uid,
                        msg_receivers: rrc.web_data.msg_receivers.to_string(),
                    },
                );
                rrc.game_data.reset_for_play_again();
                htmltemplateimplmod::open_new_local_page("#p05");
            })
            .children([ElementBuilder::new(bump, "h2")
                .attr("class", "h2_user_must_click")
                .children([text(
                    bumpalo::format!(in bump,
                        "Play again{}?",
                        ""
                    )
                    .into_bump_str(),
                )])
                .finish()])
            .finish()
    } else {
        ElementBuilder::new(bump, "div")
            .children([ElementBuilder::new(bump, "h2")
                .attr("class", "h2_user_must_wait")
                .children([text(
                    bumpalo::format!(in bump,
                        "Game Over!{}",
                        ""
                    )
                    .into_bump_str(),
                )])
                .finish()])
            .finish()
    }
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
