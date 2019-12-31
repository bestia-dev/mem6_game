// statusgameovermod.rs
//! code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;

use mem6_common::GameStatus;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///play again
pub fn div_game_over<'b>(_rrc: & RootRenderingComponent, bump: &'b Bump) -> Node<'b>
{
    //end game ,Play again?  reload webpage
    dodrio!(bump,
    <div class="div_clickable" onclick={
                move |root, vdom, _event| {
                //reload the webpage
                let window = unwrap!(web_sys::window(), "error: web_sys::window");
                let x = window.location().reload();
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
    rrc.check_invalidate_for_all_components();
}
