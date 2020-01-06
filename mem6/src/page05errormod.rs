// page05errormod.rs
//! renders the page with errors

#![allow(clippy::panic)]

//region: use statements
use crate::rootrenderingcomponentmod::RootRenderingComponent;

//use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::{bumpalo, Node, RenderContext};
use typed_html::dodrio;
//endregion

///page render
pub fn page_render<'a>(rrc: &RootRenderingComponent, cx: &mut RenderContext<'a>) -> Node<'a> {
    let bump = cx.bump;
    dodrio!(bump,
        <div>
            <h2 class="h2_user_must_wait">
                {vec![text(
                    bumpalo::format!(in bump, "error_text {}", rrc.game_data.error_text)
                        .into_bump_str(),
                    )]}
            </h2>
        </div>
    )
}
