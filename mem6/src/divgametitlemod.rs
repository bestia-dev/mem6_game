// divgametitlemod.rs
//! renders the game title

//region: use statements
use crate::rootrenderingcomponentmod::RootRenderingComponent;

//use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///the header shows the game title
pub fn div_game_title<'a>(_rrc: &RootRenderingComponent, bump: &'a Bump) -> Vec<Node<'a>> {
    let mut vec_node = Vec::new();
    vec_node.push(dodrio!(bump,
    <div>
        <h2>
            {vec![text( env!("CARGO_PKG_NAME"))]}
        </h2>
        <h4>
            {vec![text("a simple memory drinking game")]}
        </h4>
    </div>
    ));
    //return
    vec_node
}
