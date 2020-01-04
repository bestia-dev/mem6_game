// page02instructonsmod.rs
//! renders the page with instructions

//region: use statements
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::divfordebuggingmod;

use dodrio::Render;
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
        {vec![divfordebuggingmod::div_for_debugging(rrc, bump)]}
        {vec![rrc.cached_rules_and_description.render(bump)]}
    </div>
    )
}
