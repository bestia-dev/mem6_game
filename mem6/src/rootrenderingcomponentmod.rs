// rootrenderingcomponentmod.rs
//! renders the web page

//region: use, const
use crate::*;

use unwrap::unwrap;
use dodrio::{Node, Render, RenderContext};
use web_sys::WebSocket;
//endregion

/// Root Rendering Component has all
/// the data needed for play logic and rendering
pub struct RootRenderingComponent {
    /// data for web and communication
    pub web_communication: webcommunicationmod::WebCommunication,
    /// game data will be inside of Root
    pub game_data: gamedatamod::GameData,
}

/// methods
impl RootRenderingComponent {
    /// Construct a new `RootRenderingComponent` at the beginning only once.
    pub fn new(ws: WebSocket, my_ws_uid: usize) -> Self {
        let game_data = gamedatamod::GameData::new(my_ws_uid);
        let msg_receivers = prepare_msg_receivers(&game_data.players);
        let web_communication =
            webcommunicationmod::WebCommunication::new(ws, my_ws_uid, msg_receivers);

        RootRenderingComponent {
            web_communication,
            game_data,
        }
    }

    /// reset the data to play again the game
    pub fn reset_for_play_again(&mut self) {
        self.game_data.card_index_of_first_click = 0;
        self.game_data.card_index_of_second_click = 0;
        // reset points
        for x in &mut self.game_data.players {
            x.points = 0;
        }
    }
}
//endregion

//region: `Render` trait implementation on RootRenderingComponent struct
/// It is called for every Dodrio animation frame to render the vdom.
/// Only when render is scheduled after some change id the game data.
impl Render for RootRenderingComponent {
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        // let bump = cx.bump;
        // return
        // html fragment from html_template defined in # local_route
        if self.web_communication.html_template.is_empty() {
            htmltemplatemod::empty_div(cx)
        } else {
            unwrap!(htmltemplatemod::get_root_element(
                self,
                cx,
                &self.web_communication.html_template,
                htmltemplatemod::HtmlOrSvg::Html,
            ))
        }
    }
}
//endregion
