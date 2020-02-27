// divgridcontainermod.rs
//! renders the grid container with the images
//! and most important the on click event

#![allow(clippy::panic)]

//region: use, const
use crate::*;

use mem6_common::*;
use unwrap::unwrap;
use conv::{ConvUtil, ConvAsUtil};
use dodrio::{
    bumpalo::{self, Bump},
    Node,
    builder::{ElementBuilder},
};

//use wasm_bindgen::prelude::*;
//use web_sys::console;

/// fixed filename for card face down
const SRC_FOR_CARD_FACE_DOWN: &str = "img/mem_cardfacedown.png";
//endregion

/// prepare the grid container
pub fn div_grid_container<'a>(
    rrc: &RootRenderingComponent,
    bump: &'a Bump,
    max_grid_size: &Size2d,
) -> Node<'a> {
    let s_style = format!(
        "width:{}px; height:{}px;grid-template-columns: {} {} {} {};",
        max_grid_size.hor,
        max_grid_size.ver,
        if unwrap!(rrc.game_data.game_config.as_ref()).grid_items_hor >= 1 {
            "auto"
        } else {
            ""
        },
        if unwrap!(rrc.game_data.game_config.as_ref()).grid_items_hor >= 2 {
            "auto"
        } else {
            ""
        },
        if unwrap!(rrc.game_data.game_config.as_ref()).grid_items_hor >= 3 {
            "auto"
        } else {
            ""
        },
        if unwrap!(rrc.game_data.game_config.as_ref()).grid_items_hor >= 4 {
            "auto"
        } else {
            ""
        },
    );
    // return grid_container
    ElementBuilder::new(bump, "div")
        .attr("class", "grid_container")
        .attr(
            "style",
            bumpalo::format!(in bump, "{}", s_style).into_bump_str(),
        )
        .children(div_grid_items(rrc, bump))
        .finish()
}

/// prepare a vector<Node> for the Virtual Dom for 'css grid' item with <img>
/// the grid container needs only grid items. There is no need for rows and columns in 'css grid'.
#[allow(clippy::integer_arithmetic)] // end_index-1 will not overflow
pub fn div_grid_items<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Vec<Node<'a>> {
    let game_data = &rrc.game_data;

    let mut vec_grid_items: Vec<Node<'a>> = Vec::new();
    if game_data.game_config.is_some() {
        // The format 4x4 was too small for the game with multiple smartphones on the table.
        // Now I can choose different sizes gx x gy
        // grid_width x grid_height is wh cards. index goes from PlayerNUmber-1*wh+1 to Player
        // the count of cards can now be not divisible with 2 for card pairs.
        // so I need to make a different last card that is not clickable.

        // ((game_data.my_player_number - 1) * grid_width*grid_height) + 1
        let (start_index, end_index) = game_data.grid_start_end_index();
        for x in start_index..=end_index {
            let index: usize = x;
            // region: prepare variables and closures for inserting into vdom
            let img_src = match unwrap!(
                game_data.card_grid_data.get(index),
                "match game_data.card_grid_data.get(index) {} startindex {} endindex {} vec_card.len {}",
                index,
                start_index,
                end_index,
                game_data.card_grid_data.len()
            )
            .status
            {
                CardStatusCardFace::Down => bumpalo::format!(in bump, "content/{}/{}",
                                        game_data.game_name,
                                        SRC_FOR_CARD_FACE_DOWN)
                .into_bump_str(),
                CardStatusCardFace::UpTemporary | CardStatusCardFace::UpPermanently => {
                    bumpalo::format!(in bump, "content/{}/img/{}",
                    game_data.game_name,
                    unwrap!(
                        unwrap!(game_data.game_config.as_ref())
                        .img_filename.get(
                            unwrap!(game_data.card_grid_data.get(index))
                            .card_number_and_img_src
                        ))
                    )
                    .into_bump_str()
                }
            };

            let img_id =
                bumpalo::format!(in bump, "img{:02}",unwrap!(game_data.card_grid_data.get(index),
                    "game_data.card_grid_data.get(index)").card_index_and_id)
                .into_bump_str();

            let opacity = if img_src
                == format!("content/{}/{}", game_data.game_name, SRC_FOR_CARD_FACE_DOWN)
            {
                bumpalo::format!(in bump, "opacity:{}", 0.2).into_bump_str()
            } else {
                bumpalo::format!(in bump, "opacity:{}", 1).into_bump_str()
            };
            // endregion

            // creating grid_width*grid_height <div> in loop
            let grid_item_bump = div_grid_item(rrc, bump, img_src, img_id, opacity);
            vec_grid_items.push(grid_item_bump);
        }
    }

    // return
    vec_grid_items
}
/// on click is the most important part and here is more or less isolated
pub fn div_grid_item<'a>(
    rrc: &RootRenderingComponent,
    bump: &'a Bump,
    img_src: &str,
    img_id: &str,
    opacity: &str,
) -> Node<'a> {
    match rrc.game_data.game_status {
        GameStatus::Status1stCard => ElementBuilder::new(bump, "div")
            .attr("class", "grid_item")
            .children([ElementBuilder::new(bump, "img")
                .attr("class", "grid_item_img")
                .attr(
                    "src",
                    bumpalo::format!(in bump, "{}", img_src).into_bump_str(),
                )
                .attr(
                    "id",
                    bumpalo::format!(in bump, "{}", img_id).into_bump_str(),
                )
                .attr(
                    "style",
                    bumpalo::format!(in bump, "{}", opacity).into_bump_str(),
                )
                .on("click", move |root, vdom, event| {
                    status1stcardmod::on_click_img_status1st(root, &vdom, &event);
                })
                .finish()])
            .finish(),
        GameStatus::Status2ndCard => ElementBuilder::new(bump, "div")
            .attr("class", "grid_item")
            .children([ElementBuilder::new(bump, "img")
                .attr("class", "grid_item_img")
                .attr(
                    "src",
                    bumpalo::format!(in bump, "{}", img_src).into_bump_str(),
                )
                .attr(
                    "id",
                    bumpalo::format!(in bump, "{}", img_id).into_bump_str(),
                )
                .attr(
                    "style",
                    bumpalo::format!(in bump, "{}", opacity).into_bump_str(),
                )
                .on("click", move |root, vdom, event| {
                    status2ndcardmod::on_click_img_status2nd(root, &vdom, &event);
                })
                .finish()])
            .finish(),
        mem6_common::GameStatus::StatusStartPage
        | mem6_common::GameStatus::StatusJoined
        | mem6_common::GameStatus::StatusDrink
        | mem6_common::GameStatus::StatusTakeTurn
        | mem6_common::GameStatus::StatusGameOver
        | mem6_common::GameStatus::StatusReconnect
        | mem6_common::GameStatus::StatusWaitingAckMsg => ElementBuilder::new(bump, "div")
            .attr("class", "grid_item")
            .children([ElementBuilder::new(bump, "img")
                .attr("class", "grid_item_img")
                .attr(
                    "src",
                    bumpalo::format!(in bump, "{}", img_src).into_bump_str(),
                )
                .attr(
                    "id",
                    bumpalo::format!(in bump, "{}", img_id).into_bump_str(),
                )
                .attr(
                    "style",
                    bumpalo::format!(in bump, "{}", opacity).into_bump_str(),
                )
                .finish()])
            .finish(),
    }
}

/// play sound mp3
pub fn play_sound(rrc: &RootRenderingComponent, this_click_card_index: usize) {
    // prepare the audio element with src filename of mp3
    let src_mp3 = format!(
        "content/{}/sound/{}",
        rrc.game_data.game_name,
        unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
            .sound_filename
            .get(
                unwrap!(
                    rrc.game_data.card_grid_data.get(this_click_card_index),
                    "error this_click_card_index"
                )
                .card_number_and_img_src
            ))
    );

    websysmod::play_sound(&src_mp3);
}

/// grid width in pixels
pub fn grid_width() -> usize {
    // the size of  the visible part of the window
    let usize_inner_width = usize_window_inner_width();
    // width min: 300px, max: 600 px in between width=visible width
    // 3 columnsdelimiter 5px wide
    let grid_width: usize;
    if usize_inner_width < 300 {
        grid_width = 300;
    } else if usize_inner_width > 600 {
        grid_width = 600;
    } else {
        grid_width = usize_inner_width;
    }
    grid_width
}

/// grid height in pixels
pub fn grid_height() -> usize {
    // the size of  the visible part of the window
    let usize_inner_height = usize_window_inner_height();

    // height minimum 300, max 1000, else 0.8*visible height
    // 3 row separators 5px wide
    let grid_height: usize;
    if usize_inner_height < 300 {
        grid_height = 300;
    } else if usize_inner_height > 1000 {
        grid_height = 1000;
    } else {
        grid_height =
            unwrap!((0.8 * (unwrap!(usize_inner_height.approx_as::<f64>()))).approx_as::<usize>());
    }
    grid_height
}

/// calculate max with and height for a grid in pixels
pub fn max_grid_size(rrc: &RootRenderingComponent) -> Size2d {
    // if the game_config is None, then return full screen
    if rrc.game_data.game_config.is_none() {
        Size2d {
            hor: usize_window_inner_width_but_max_600(),
            ver: usize_window_inner_height(),
        }
    } else {
        // grid_container width and height
        let mut max_grid_width = grid_width();
        let mut max_grid_height = grid_height();
        /*
        // websysmod::debug_write(&format!(
            "inner_width {} inner_height {}",
            max_grid_width, max_grid_height
        ));
        */
        // default if not chosen
        let mut card_width = 115;
        let mut card_height = 115;
        match &rrc.game_data.game_config {
            None => (),
            Some(_x) => {
                card_width = unwrap!(rrc.game_data.game_config.clone()).card_width;
                card_height = unwrap!(rrc.game_data.game_config.clone()).card_height;
            }
        }
        /*
        // websysmod::debug_write(&format!(
            "card_width {} card_height {}",
            card_width, card_height
        ));
        */
        // ratio between width and height must stay the same
        let ratio = (unwrap!(card_height.approx_as::<f64>())
            * unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                .grid_items_ver
                .approx_as::<f64>()))
            / (unwrap!(card_width.approx_as::<f64>())
                * unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                    .grid_items_hor
                    .approx_as::<f64>()));

        if unwrap!(max_grid_width.approx_as::<f64>()) * ratio
            > unwrap!(max_grid_height.approx_as::<f64>())
        {
            max_grid_width =
                unwrap!((unwrap!(max_grid_height.approx_as::<f64>()) / ratio).approx_as::<usize>());
        } else {
            max_grid_height =
                unwrap!((unwrap!(max_grid_width.approx_as::<f64>()) * ratio).approx_as::<usize>());
        }
        /*
        // websysmod::debug_write(&format!(
            "max_grid_width {} max_grid_height {}",
            max_grid_width, max_grid_height
        ));
        */

        // return
        Size2d {
            hor: max_grid_width,
            ver: max_grid_height,
        }
    }
}

/// return window inner height
/// the size of  the visible part of the window
pub fn usize_window_inner_height() -> usize {
    let jsvalue_inner_height = unwrap!(websysmod::window().inner_height(), "window.inner_height");

    let f64_inner_height = unwrap!(
        jsvalue_inner_height.as_f64(),
        "jsValue_inner_height.as_f64()"
    );
    let usize_inner_height: usize = unwrap!(f64_inner_height.approx());
    // return
    usize_inner_height
}

/// return window inner width
/// the size of  the visible part of the window
pub fn usize_window_inner_width() -> usize {
    let jsvalue_inner_width = unwrap!(websysmod::window().inner_width(), "window.inner_width");

    let f64_inner_width = unwrap!(
        jsvalue_inner_width.as_f64(),
        "jsValue_inner_width.as_string()"
    );
    let usize_inner_width: usize = unwrap!(f64_inner_width.approx());
    // return
    usize_inner_width
}

/// return window inner width, but maximum 600px
/// the size of  the visible part of the window
pub fn usize_window_inner_width_but_max_600() -> usize {
    let x = usize_window_inner_width();
    if x > 600 {
        // return
        600
    } else {
        // return
        x
    }
}
