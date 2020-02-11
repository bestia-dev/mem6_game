// statusgameovermod.rs
//! code flow from this status

#![allow(clippy::panic)]

//region: use
use crate::*;
use mem6_common::*;

use unwrap::unwrap;
use dodrio::{
    builder::text,
    bumpalo::{self, Bump},
    Node,
};
use typed_html::dodrio;
//endregion

///play again
pub fn div_game_over<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    //game over
    // only the first player can choose Play again?
    // other players are already joined to the group
    if rrc.game_data.my_player_number == 1 {
        dodrio!(bump,
            <div class="div_clickable" onclick={
                        move |root, vdom, _event| {
                        let rrc = root.unwrap_mut::<RootRenderingComponent>();
                        let window = unwrap!(web_sys::window(), "error: web_sys::window");
                        websocketcommunicationmod::ws_send_msg(
                            &rrc.game_data.ws,
                            &WsMessage::MsgPlayAgain {
                                my_ws_uid: rrc.game_data.my_ws_uid,
                                players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                            },
                        );
                        rrc.reset_for_play_again();
                        fncallermod::open_new_local_page("#p02");
                    }}>
                <h2 class="h2_user_can_click">
                        {vec![text(
                            bumpalo::format!(in bump, "Play again{}?", "").into_bump_str(),
                        )]}
                </h2>
            </div>
        )
    } else {
        dodrio!(bump,
            <div >
                <h2 class="h2_user_must_wait">
                        {vec![text(
                            bumpalo::format!(in bump, "Game Over!{}", "").into_bump_str(),
                        )]}
                </h2>
            </div>
        )
    }
}

///on msg game over
pub fn on_msg_game_over(rrc: &mut RootRenderingComponent) {
    //The game is over.
    rrc.game_data.game_status = GameStatus::StatusGameOver;
}

///on msg play again
pub fn on_msg_play_again(rrc: &mut RootRenderingComponent) {
    //The first players can choose Play again and send to others.
    rrc.game_data.game_status = GameStatus::StatusJoined;
    rrc.reset_for_play_again();
    let group_id = fncallermod::group_id_joined(rrc);
    fncallermod::open_new_local_page(&format!("#p04.{}", group_id));
}
