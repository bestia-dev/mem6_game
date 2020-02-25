// rootrenderingcomponentmod.rs
//! renders the web page

//region: use, const
use crate::*;
use crate::htmltemplatemod::HtmlTemplating;

use unwrap::unwrap;
use dodrio::{Node, Render, RenderContext};
use web_sys::WebSocket;
//endregion

/// Root Rendering Component has all
/// the data needed for play logic and rendering
pub struct RootRenderingComponent {
    /// data for web and communication
    pub web_data: webdatamod::WebData,
    /// game data will be inside of Root
    pub game_data: gamedatamod::GameData,
    /// videos links for fun
    pub videos: Vec<String>,
    /// audio for fun
    pub audio: Vec<String>,
}

/// impl
impl RootRenderingComponent {
    /// Construct a new `RootRenderingComponent` at the beginning only once.
    pub fn new(ws: WebSocket, my_ws_uid: usize) -> Self {
        let game_data = gamedatamod::GameData::new(my_ws_uid);
        let msg_receivers = game_data.prepare_msg_receivers();
        let web_data = webdatamod::WebData::new(ws, my_ws_uid, msg_receivers);

        RootRenderingComponent {
            web_data,
            game_data,
            videos: vec![],
            audio: vec![],
        }
    }
}

///`Render` trait implementation on RootRenderingComponent struct
/// It is called for every Dodrio animation frame to render the vdom.
/// Only when render is scheduled after some change id the game data.
impl Render for RootRenderingComponent {
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        // let bump = cx.bump;
        // return
        // html fragment from html_template defined in # local_route
        if self.web_data.html_template.is_empty() {
            htmltemplatemod::empty_div(cx)
        } else {
            //I must add use crate::htmltemplatemod::HtmlTemplating;
            // to allow this trait to be used here on self
            unwrap!(self.get_root_node(
                cx,
                &self.web_data.html_template,
                htmltemplatemod::HtmlOrSvg::Html,
            ))
        }
    }
}
