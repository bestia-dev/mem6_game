// statusgameovermod.rs
//! code flow from this status

#![allow(clippy::panic)]

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::*;
use mem6_common::GameStatus;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///play again
pub fn div_game_over<'a>(_rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    //end game ,Play again?  reload webpage
    dodrio!(bump,
    <div class="div_clickable" onclick={
                move |root, vdom, _event| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                let window = unwrap!(web_sys::window(), "error: web_sys::window");
                //the first player go to the start group page
                //other players join the group
                if rrc.game_data.my_player_number==1{
                    fncallermod::open_new_local_page("#p02");
                }
                else{
                    let group_id=fncallermod::group_id_joined(rrc);
                    fncallermod::open_new_local_page(&format!("#p04.{}",group_id));
                }
            }}>
        <h2 class="h2_user_can_click">
                {vec![text(
                    //Play again?
                    bumpalo::format!(in bump, "Game Over! Play again{}?", "").into_bump_str(),
                )]}
        </h2>
    </div>
    )
}

///msg player click
pub fn on_msg_game_over(rrc: &mut RootRenderingComponent) {
    //logmod::debug_write("on_msg_game_over");
    //The game is over and the question Play again?
    rrc.game_data.game_status = GameStatus::StatusGameOver;

}
