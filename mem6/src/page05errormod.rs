// page02nicknamemod.rs
//! renders the page with errors

//region: use statements
use crate::rootrenderingcomponentmod::RootRenderingComponent;

//use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///page render
pub fn page_render<'b>(rrc: &RootRenderingComponent, bump: &'b Bump) -> Node<'b> {
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
