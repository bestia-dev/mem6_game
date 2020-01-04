// page02nicknamemod.rs
//! renders the first page with nickname and choose group

//region: use statements
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::divgametitlemod;
use crate::divnicknamemod;
use crate::divplayeractionsmod;

//use dodrio::Render;
//use unwrap::unwrap;
//use dodrio::builder::text;
use dodrio::bumpalo::{Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///page render
pub fn page_render<'b>(rrc: &'b RootRenderingComponent, bump: &'b Bump) -> Node<'b> {
    dodrio!(bump,
    <div class= "m_container" >
        {divgametitlemod::div_game_title(rrc, bump)}
        {vec![divnicknamemod::div_nickname_input(rrc,bump)]}
        {vec![divplayeractionsmod::div_player_actions_from_game_status(rrc, bump)]}
    </div>
    )
}
