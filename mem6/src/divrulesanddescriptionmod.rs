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
const GAME_RULES1: &str = "This multi-player game is prepared for \
android smartphones with internet connection. \
All players must be in physical proximity around a table. \
Open this link 
https://bestia.dev/mem6
on all smartphones. \
Use the QR code on the bottom of this text. 
For better user experience 
click on ";

const GAME_RULES2: &str = "On the start page write your nickname. \
Now put all the smartphones on the table near to each other, \
so all players can see them and touch \
them. All smartphones combined will be used as a board game. \
The first player invites the others and chooses a kind of game visuals: \
alphabet, animal, playing cards,...  \
Other players can then accept the invitation.  \
If there are some communication sync issues, \
reload the webpage simultaneously on all smartphones. \
Just pool the screen down slightly and let go when the reload circle is visible. \
Player1 will see how many players have accepted. Then he starts the game. \
On the screen under the grid are clear instructions which player plays and which waits. \
The green color calls for player action, the red means wait and the orange is informational. \
Player1 flips over two cards with two clicks. This cards can be on any smartphone. \
The cards are accompanied by sounds and text on the screen. \
If the cards do not match, the next player will click on 'Click here to Take your turn'. \
The cards are than flipped back face down. \
Then it is his turn and he clicks to flip over his two cards. \
If the cards match, they are left face up permanently and the player receives a point. \
He continues to play, he opens the next two cards. \
The game is over when all the cards are permanently face up.
Good luck and enjoy !";

///game description
const GAME_DESCRIPTION:& str = "The game mem6 is about learning programming in Rust Wasm/WebAssembly with Dodrio Virtual Dom, WebSockets communication and PWA (Progressive Web App)";

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
                    "https://github.com /LucianoBestia /mem6_game{}", "").into_bump_str(),)]}
                </a>
            </h6>
            <h2>
            {vec![text(
                bumpalo::format!(in bump, "mem6 game rules: {}", "").into_bump_str(),
            )]}
            </h2>
            <h4>
                {text_with_br_newline(GAME_RULES1, bump)}
            </h4>
            {divfullscreenmod::div_fullscreen(self.is_fullscreen,bump)}
            <h4>
                {text_with_br_newline(GAME_RULES2, bump)}
            </h4>
            <img src="images/qr_mem6.png">
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
