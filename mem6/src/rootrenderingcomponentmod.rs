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
    /// local # hash route
    pub local_route: String,
    /// downloaded html template
    pub html_template: String,
    ///game data will be inside of Root
    pub game_data: gamedatamod::GameData,
}

///methods
impl RootRenderingComponent {
    /// Construct a new `RootRenderingComponent` at the beginning only once.
    pub fn new(ws: WebSocket, my_ws_uid: usize) -> Self {
        let game_data = gamedatamod::GameData::new(ws, my_ws_uid);

        RootRenderingComponent {
            local_route: "".to_owned(),
            html_template: "".to_owned(),
            game_data,
        }
    }
    ///reset the data to play again the game
    pub fn reset_for_play_again(&mut self) {
        self.game_data.card_index_of_first_click = 0;
        self.game_data.card_index_of_second_click = 0;
        //reset points
        for x in &mut self.game_data.players {
            x.points = 0;
        }
    }
}
//endregion

//region: `Render` trait implementation on RootRenderingComponent struct
///It is called for every Dodrio animation frame to render the vdom.
///Only when render is scheduled after some change id the game data.
impl Render for RootRenderingComponent {
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        let bump = cx.bump;
        //return
        // html fragment from html_template defined in # local_route
        if self.html_template.is_empty() {
            htmltemplatemod::empty_div(cx)
        } else {
            unwrap!(htmltemplatemod::get_root_element(
                self,
                cx,
                &self.html_template,
                htmltemplatemod::HtmlOrSvg::Html,
                &fncallermod::call_function_string,
                &fncallermod::call_function_node,
                &fncallermod::call_listener,
            ))
        }
    }
}
//endregion
