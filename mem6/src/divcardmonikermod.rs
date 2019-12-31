// divcardmonikermod.rs
//! renders the card moniker (card name/title)

//region: use statements
use crate::rootrenderingcomponentmod::RootRenderingComponent;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///the header can show only the game title or two card monikers. Not everything together.
pub fn div_grid_card_moniker<'b>(rrc: &RootRenderingComponent, bump: &'b Bump) -> Vec<Node<'b>> {
    //this game_data mutable reference is dropped on the end of the function
    let game_data = &rrc.game_data;
    let mut vec_node = Vec::new();
    //if the card_monikers are visible, than don't show GameTitle, because there is not
    //enought space on smartphones
        let left_text = unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
            .card_moniker
            .get(
                unwrap!(game_data
                    .card_grid_data
                    .get(game_data.card_index_of_first_click))
                .card_number_and_img_src
            ))
        .to_string();
        let left_text_len = left_text.len();
        let left_fontsize = calc_font_size(left_text_len);
        let left_style_string =
            bumpalo::format!(in bump, "font-size:{}rem;", left_fontsize).into_bump_str();

        let right_text = unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
            .card_moniker
            .get(
                unwrap!(game_data
                    .card_grid_data
                    .get(game_data.card_index_of_second_click))
                .card_number_and_img_src
            ))
        .to_string();
        let right_text_len = right_text.len();
        let right_fontsize = calc_font_size(right_text_len);
        let right_style_string =
            bumpalo::format!(in bump, "font-size:{}rem;", right_fontsize).into_bump_str();
        vec_node.push(dodrio!(bump,
        <div class= "grid_container_header" style="grid-template-columns: 50% 50%;\
            min-height: 60px; vertical-align: middle;">
            <div id="card_moniker_left" class= "grid_item" style={left_style_string} >
                {vec![text(bumpalo::format!(in bump, "{}",left_text).into_bump_str())]}
            </div>
            <div id="card_moniker_right" class= "grid_item" style={right_style_string} >
                {vec![text(bumpalo::format!(in bump, "{}", right_text).into_bump_str())]}
            </div>
        </div>
        ));
    //return
    vec_node
}

/// when the lenght is bigger, the fontsize get smaller
/// if the len is 10 the fontsize is 3.0rem, if the len is 20 the fontsize is 1.5rem
/// this means that the 80 is constant:  10*3.0=30 20*1.5=30
#[allow(
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::cast_precision_loss
)]
fn calc_font_size(text_len: usize) -> f64 {
    if text_len < 10 {
        3.0
    } else {
        30.0 / text_len as f64
    }
}
