// divrulesanddescriptionmod.rs
//! renders the div that shows rules and descriptions
//! All is a static content. Great for implementing dodrio cache.

//region: use
use crate::gamedatamod::GameData;
use crate::divfullscreenmod;

use dodrio::builder::{br, text};
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use typed_html::dodrio;
//endregion

///Text of game rules.
///Multiline string literal just works.
///End of line in the code is simply and intuitively end of line in the string.
///The special character \ at the end of the line in code means that it is NOT the end of the line for the string.
///The escape sequence \n means end of line also. For doublequote simply \" .
const GAME_RULES1: &str = " ";

const GAME_RULES2: &str = "";

///game description
const GAME_DESCRIPTION: &str = "";

///Render Component: The static parts can be cached easily.
pub struct RulesAndDescription {
    /// to not check it all the time
    pub is_fullscreen: bool,
}

impl RulesAndDescription {
    ///constructor
    pub fn new() -> Self {
        RulesAndDescription {
            is_fullscreen: false,
        }
    }

    pub fn update_intern_cache(&mut self, game_data: &GameData) -> bool {
        let mut is_invalidated;
        is_invalidated = false;

        if self.is_fullscreen != game_data.is_fullscreen {
            self.is_fullscreen = game_data.is_fullscreen;
            is_invalidated = true;
        }
        is_invalidated
    }
}

impl Render for RulesAndDescription {
    ///This rendering will be rendered and then cached . It will not be rerendered untill invalidation.
    ///In this case I don't need to invalidate because it is a static content.
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        dodrio!(bump,
        <div>
            <h4>
                {text_with_br_newline(GAME_DESCRIPTION,bump)}
            </h4>
             <h6>
                /*TODO: tried to add rel="noreferrer", but the dodrio! macro doesn't understand that */
                <a href= "https://github.com/LucianoBestia/mem6_game" target="_blank" style="color:#74bbfb">
                    {vec![text(bumpalo::format!(in bump,
                    "github mem6{}", "").into_bump_str(),)]}
                </a>
            </h6>
            /*
            <h2>
            {vec![text(
                bumpalo::format!(in bump, "mem6 game rules: {}", "").into_bump_str(),
            )]}
            </h2>
            <h4>
                {text_with_br_newline(GAME_RULES1, bump)}
            </h4>
            */
            {divfullscreenmod::div_fullscreen(self.is_fullscreen,bump)}
            /*
            <h4>
                {text_with_br_newline(GAME_RULES2, bump)}
            </h4>
            */
            <img class="center" src="images/qr_mem6.png" >
            </img>
        </div>
        )
    }
}

///change the newline lines ending into <br> node
fn text_with_br_newline<'a>(txt: &'a str, bump: &'a Bump) -> Vec<Node<'a>> {
    let mut vec_text_node = Vec::new();
    let spl = txt.lines();
    for part in spl {
        vec_text_node.push(text(part));
        vec_text_node.push(br(bump).finish());
    }
    vec_text_node
}
