// page02instructonsmod.rs
//! renders the page with instructions

#![allow(clippy::panic)]

//region: use statements
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::divfordebuggingmod;

use dodrio::Render;
//use unwrap::unwrap;
//use dodrio::builder::text;
use dodrio::{Node, RenderContext};
use typed_html::dodrio;
//endregion

///page render
pub fn page_render<'a>(rrc: &'a RootRenderingComponent, cx: &mut RenderContext<'a>) -> Node<'a> {
    let bump = cx.bump;
    dodrio!(bump,
    <div class= "m_container" >
        {vec![divfordebuggingmod::div_for_debugging(rrc, cx.bump)]}
        {vec![rrc.cached_rules_and_description.render(cx)]}
    </div>
    )
}
