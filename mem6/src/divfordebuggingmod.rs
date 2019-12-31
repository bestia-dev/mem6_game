// divfordebuggingmod.rs
//! information for debugging

//region: use, const
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::sessionstoragemod;
use crate::websocketreconnectmod;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///information for debugging
#[allow(dead_code)]
pub fn div_for_debugging<'a>(rrc: &'a RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    if rrc.game_data.show_debug_info {
        dodrio!(bump,
        <div >
            <pre style="color: white; white-space: pre-wrap; word-break: break-all;">
                {vec![text(
                    bumpalo::format!(in bump, "debug info {}:\n{}",
                        env!("CARGO_PKG_VERSION"),
                        sessionstoragemod::get_debug_text()
                        ).into_bump_str()
                )]}
            </pre>
            {vec![websocketreconnectmod::div_reconnect(rrc, bump)]}
        </div>
        )
    } else {
        dodrio!(bump,
        <div>
            <div class="div_clickable" onclick={move |root, vdom, _event| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                rrc.game_data.show_debug_info=true;
                vdom.schedule_render();
            }}>
                <h5 style="color:orange">
                    {vec![text(
                    bumpalo::format!(in bump, "Debug info {}", env!("CARGO_PKG_VERSION"))
                    .into_bump_str(),
                    )]}
                </h5>
            </div>
        </div>
        )
    }
}
