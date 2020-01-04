// page03gamemod.rs
//! renders the page with game grid

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
use dodrio::bumpalo::{Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///page render
pub fn page_render<'b>(
    rrc: &'b RootRenderingComponent,
    bump: &'b Bump,
    xmax_grid_size: gamedatamod::Size2d,
) -> Node<'b> {
    dodrio!(bump,
        <div class= "m_container" >
        {vec![divgridcontainermod::div_grid_container(rrc,bump,&xmax_grid_size)]}
        {divcardmonikermod::div_grid_card_moniker(rrc, bump)}
        {vec![rrc.cached_players_and_scores.render(bump)]}
        {vec![divplayeractionsmod::div_player_actions_from_game_status(rrc, bump)]}
        {divgametitlemod::div_game_title(rrc, bump)}
        {vec![divfordebuggingmod::div_for_debugging(rrc, bump)]}
    </div>
    )
}
