// statuswaitingackmsgmod.rs
//! code flow from this status

#![allow(clippy::panic)]

//region: use
use crate::*;

//use unwrap::unwrap;
use dodrio::{
    builder::{ElementBuilder, text},
    bumpalo::{self, Bump},
    Node,
};
//use typed_html::dodrio;
//endregion

/// waiting ack msg
pub fn div_waiting_ack_msg<'a>(_rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    /*
    dodrio!(bump,
    <div>
        <h2 class="h2_user_must_wait">
                {vec![text(
                    bumpalo::format!(in bump, "slow network{}", "").into_bump_str(),
                )]}
        </h2>
    </div>
    )
    */
    ElementBuilder::new(bump, "div")
        .children([ElementBuilder::new(bump, "h2")
            .attr("class", "h2_user_must_wait")
            .children([text(
                bumpalo::format!(in bump,
                    "slow network{}",
                    ""
                )
                .into_bump_str(),
            )])
            .finish()])
        .finish()
}
