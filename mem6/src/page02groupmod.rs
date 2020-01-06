// page02groupmod.rs
//! renders the group page

#![allow(clippy::panic)]

//region: use statements
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::divgametitlemod;
use crate::divnicknamemod;
use crate::divplayeractionsmod;

//use dodrio::Render;
//use unwrap::unwrap;
//use dodrio::builder::text;
use dodrio::{Node, RenderContext};
use typed_html::dodrio;
//endregion

///page render
pub fn page_render<'a>(rrc: &RootRenderingComponent, cx: &mut RenderContext<'a>) -> Node<'a> {
    let bump = cx.bump;
    dodrio!(bump,
    <div class= "m_container" >
        {divgametitlemod::div_game_title(rrc, cx.bump)}
        {vec![divnicknamemod::div_nickname_input(rrc,cx.bump)]}
        {vec![divplayeractionsmod::div_player_actions_from_game_status(rrc, cx.bump)]}
    </div>
    )
}
