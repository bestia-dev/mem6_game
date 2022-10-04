// rootrenderingcomponentmod.rs
//! renders the web page

// region: use, const
use crate::htmltemplatemod::HtmlTemplating;
use crate::*;

use dodrio::{Node, Render, RenderContext};
use unwrap::unwrap;
// endregion

/// Root Rendering Component has all
/// the data needed for play logic and rendering
pub struct RootRenderingComponent {
    /// data for web and communication
    pub web_data: webdatamod::WebData,
    /// game data will be inside of Root
    pub game_data: gamedatamod::GameData,
}

/// impl
impl RootRenderingComponent {
    /// Construct a new `RootRenderingComponent` at the beginning only once.
    pub fn new(my_ws_uid: usize) -> Self {
        let game_data = gamedatamod::GameData::new(my_ws_uid);
        let msg_receivers_json = game_data.prepare_json_msg_receivers();
        let web_data = webdatamod::WebData::new(my_ws_uid, msg_receivers_json);

        RootRenderingComponent {
            web_data,
            game_data,
        }
    }
}

///`Render` trait implementation on RootRenderingComponent struct
/// It is called for every Dodrio animation frame to render the vdom.
/// Only when render is scheduled after some change id the game data.
impl<'a> Render<'a> for RootRenderingComponent {
    fn render(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        // let bump = cx.bump;
        // return
        // html fragment from html_template defined in # local_route
        if self.web_data.html_template.is_empty() {
            htmltemplatemod::empty_div(cx)
        } else {
            // i must add use crate::htmltemplatemod::HtmlTemplating;
            // to allow this trait to be used here on self
            unwrap!(self.render_template(
                cx,
                &self.web_data.html_template,
                htmltemplatemod::HtmlOrSvg::Html,
            ))
        }
    }
}
