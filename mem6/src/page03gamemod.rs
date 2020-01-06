// page03gamemod.rs
//! renders the page with game grid

#![allow(clippy::panic)]

//region: use statements
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::divgridcontainermod;
use crate::divcardmonikermod;
use crate::divplayeractionsmod;
use crate::divfordebuggingmod;
use crate::divgametitlemod;
use crate::gamedatamod;

use dodrio::Render;
//use unwrap::unwrap;
//use dodrio::builder::text;
use dodrio::{Node, RenderContext};
use typed_html::dodrio;
//endregion

///page render
pub fn page_render<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
    xmax_grid_size: &gamedatamod::Size2d,
) -> Node<'a> {
    let bump = cx.bump;
    dodrio!(bump,
        <div class= "m_container" >
        {vec![divgridcontainermod::div_grid_container(rrc,cx.bump,&xmax_grid_size)]}
        {divcardmonikermod::div_grid_card_moniker(rrc, cx.bump)}
        {vec![rrc.cached_players_and_scores.render(cx)]}
        {vec![divplayeractionsmod::div_player_actions_from_game_status(rrc, cx.bump)]}
        {divgametitlemod::div_game_title(rrc, cx.bump)}
        //{vec![divfordebuggingmod::div_for_debugging(rrc, cx.bump)]}
    </div>
    )
}
