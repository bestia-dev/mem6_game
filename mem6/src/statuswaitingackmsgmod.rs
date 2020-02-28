// statuswaitingackmsgmod.rs
//! code flow from this status

#![allow(clippy::panic)]

//region: use
use crate::*;

use unwrap::unwrap;
use dodrio::{RenderContext, Node};
use crate::htmltemplatemod::HtmlTemplating;
//endregion

/// waiting ack msg
pub fn div_waiting_ack_msg<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
) -> Node<'a> {
    let html_template = r#"
    <div >
        <h2 class="h2_user_must_wait">
            Slow network !
        </h2>
    </div>"#;
    unwrap!(rrc.prepare_node_from_template(cx, html_template, htmltemplatemod::HtmlOrSvg::Html))
}
