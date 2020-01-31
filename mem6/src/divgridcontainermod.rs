// divgridcontainermod.rs
//! renders the grid container with the images
//! and most important the on click event

#![allow(clippy::panic)]

//region: use, const
use crate::gamedatamod::{CardStatusCardFace, Size2d};
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::*;

use mem6_common::GameStatus;

use unwrap::unwrap;
use conv::{ConvUtil};
use conv::{ConvAsUtil};
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
                          //use web_sys::console;

///fixed filename for card face down
const SRC_FOR_CARD_FACE_DOWN: &str = "img/mem_cardfacedown.png";
//endregion

///prepare the grid container
pub fn div_grid_container<'a>(
    rrc: &RootRenderingComponent,
    bump: &'a Bump,
    max_grid_size: &Size2d,
) -> Node<'a> {
    let xstyle = format!(
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
    let grid_container = dodrio!(bump,
        <div class= "grid_container" style={xstyle}>
            {div_grid_items(rrc, bump)}
        </div>
    );
    //return
    grid_container
}

///prepare a vector<Node> for the Virtual Dom for 'css grid' item with <img>
///the grid container needs only grid items. There is no need for rows and columns in 'css grid'.
#[allow(clippy::integer_arithmetic)] // end_index-1 will not overflow
pub fn div_grid_items<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Vec<Node<'a>> {
    //this game_data mutable reference is dropped on the end of the function
    let game_data = &rrc.game_data;

    let mut vec_grid_items: Vec<Node<'a>> = Vec::new();
    if game_data.game_config.is_some() {
        //The format 4x4 was too small for the game with multiple smartphones on the table.
        //Now I can choose different sizes gx x gy
        //grid_width x grid_height is wh cards. index goes from PlayerNUmber-1*wh+1 to Player
        //the count of cards can now be not divisible with 2 for card pairs.
        //so I need to make a different last card that is not clickable.

        //((game_data.my_player_number - 1) * grid_width*grid_height) + 1
        let start_index = unwrap!(unwrap!((unwrap!(game_data.my_player_number.checked_sub(1)))
            .checked_mul(unwrap!(unwrap!(game_data.game_config.as_ref())
                .grid_items_hor
                .checked_mul(unwrap!(game_data.game_config.as_ref()).grid_items_ver))))
        .checked_add(1));
        let mut end_index =
            unwrap!(game_data
                .my_player_number
                .checked_mul(unwrap!(unwrap!(game_data.game_config.as_ref())
                    .grid_items_hor
                    .checked_mul(unwrap!(game_data.game_config.as_ref()).grid_items_ver))));

        //the count of cards can now be not divisible with 2 for card pairs.
        //so I need to make a different last card that is not clickable.
        if end_index >= game_data.card_grid_data.len() {
            end_index -= 1;
        }

        /*
                logmod::debug_write(&format!(
                    "div_grid_items: my_player_number {} start_index {} end_index {} card_grid_data.len {}",
                    &rrc.game_data.my_player_number,
                    start_index,
                    end_index,
                    game_data.card_grid_data.len()
                ));
        */

        for x in start_index..=end_index {
            let index: usize = x;
            //region: prepare variables and closures for inserting into vdom
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
            //endregion

            //creating grid_width*grid_height <div> in loop
            let grid_item_bump = div_grid_item(rrc, bump, img_src, img_id, opacity);
            vec_grid_items.push(grid_item_bump);
        }
    }

    //return
    vec_grid_items
}
///on click is the most important part and here is more or less isolated
pub fn div_grid_item<'a>(
    rrc: &RootRenderingComponent,
    bump: &'a Bump,
    img_src: &str,
    img_id: &str,
    opacity: &str,
) -> Node<'a> {
    match rrc.game_data.game_status {
        GameStatus::Status1stCard => {
            dodrio!(bump,
            <div class= "grid_item">
            <img class= "grid_item_img" src={img_src} id={img_id} style={opacity} onclick={move |root, vdom, event| {
                logmod::debug_write("img click");
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                // If the event's target is our image...
                let img = match event
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                {
                    None => return,
                    //?? Don't understand what this does. The original was written for Input element.
                    Some(input) => input,
                };
                //id attribute of image html element is prefixed with img ex. "img12"
                let this_click_card_index = unwrap!(
                    (unwrap!(img.id().get(3..), "error slicing")).parse::<usize>(),
                    "error parse img id to usize"
                );
                //click is usefull only on facedown cards
                if unwrap!(
                    rrc.game_data.card_grid_data.get(this_click_card_index),
                    "error this_click_card_index"
                ).status.as_ref()==CardStatusCardFace::Down.as_ref(){
                    status1stcardmod::on_click_1st_card(rrc, &vdom, this_click_card_index);
                    // Finally, re-render the component on the next animation frame.
                    vdom.schedule_render();
                }
            }}>
            </img>
            </div>
            )
        }
        GameStatus::Status2ndCard => {
            dodrio!(bump,
            <div class= "grid_item">
            <img class= "grid_item_img" src={img_src} id={img_id} style={opacity} onclick={move |root, vdom, event| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                // If the event's target is our image...
                let img = match event
                    .target()
                    .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
                {
                    None => return,
                    //?? Don't understand what this does. The original was written for Input element.
                    Some(input) => input,
                };
                //id attribute of image html element is prefixed with img ex. "img12"
                let this_click_card_index = unwrap!(
                    (unwrap!(img.id().get(3..), "error slicing")).parse::<usize>(),
                    "error parse img id to usize"
                );
                //click is usefull only on facedown cards
                if unwrap!(
                    rrc.game_data.card_grid_data.get(this_click_card_index),
                    "error this_click_card_index"
                ).status.as_ref()==CardStatusCardFace::Down.as_ref(){
                    status2ndcardmod::on_click_2nd_card(rrc, &vdom, this_click_card_index);
                    // Finally, re-render the component on the next animation frame.
                    vdom.schedule_render();
                }
            }}>
            </img>
            </div>
            )
        }
        mem6_common::GameStatus::StatusStartPage
        | mem6_common::GameStatus::StatusJoined
        | mem6_common::GameStatus::StatusTakeTurnBegin
        | mem6_common::GameStatus::StatusTakeTurnEnd
        | mem6_common::GameStatus::StatusGameOver
        | mem6_common::GameStatus::StatusReconnect
        | mem6_common::GameStatus::StatusWaitingAckMsg => dodrio!(bump,
            <div class= "grid_item">
                <img class= "grid_item_img" src={img_src} id={img_id} style={opacity} >
                </img>
            </div>
        ),
    }
}

/// play sound mp3
pub fn play_sound(rrc: &RootRenderingComponent, this_click_card_index: usize) {
    //prepare the audio element with src filename of mp3
    let audio_element = web_sys::HtmlAudioElement::new_with_src(
        format!(
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
        )
        .as_str(),
    );

    //play() return a Promise in JSValue. That is too hard for me to deal with now.
    unwrap!(
        unwrap!(audio_element, "Error: HtmlAudioElement new.").play(),
        "Error: HtmlAudioElement.play() "
    );
}

///grid width in pixels
pub fn grid_width() -> usize {
    //the size of  the visible part of the window
    let usize_inner_width = usize_window_inner_width();
    //width min: 300px, max: 600 px in between width=visible width
    //3 columnsdelimiter 5px wide
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

///grid height in pixels
pub fn grid_height() -> usize {
    //the size of  the visible part of the window
    let usize_inner_height = usize_window_inner_height();

    //height minimum 300, max 1000, else 0.8*visible height
    //3 row separetors 5px wide
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

///calculate max with and height for a grid in pixels
pub fn max_grid_size(rrc: &RootRenderingComponent) -> Size2d {
    //if the game_config is None, then return full screen
    if rrc.game_data.game_config.is_none() {
        Size2d {
            hor: usize_window_inner_width_but_max_600(),
            ver: usize_window_inner_height(),
        }
    } else {
        //grid_container width and height
        let mut max_grid_width = grid_width();
        let mut max_grid_height = grid_height();
        /*
        logmod::debug_write(&format!(
            "inner_width {} inner_height {}",
            max_grid_width, max_grid_height
        ));
        */
        //default if not choosen
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
        logmod::debug_write(&format!(
            "card_width {} card_height {}",
            card_width, card_height
        ));
        */
        //ratio between width and height must stay the same
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
        logmod::debug_write(&format!(
            "max_grid_width {} max_grid_height {}",
            max_grid_width, max_grid_height
        ));
        */

        //return
        Size2d {
            hor: max_grid_width,
            ver: max_grid_height,
        }
    }
}

/// return window inner height
/// the size of  the visible part of the window
pub fn usize_window_inner_height() -> usize {
    let window = unwrap!(web_sys::window(), "error: web_sys::window");
    let jsvalue_inner_height = unwrap!(window.inner_height(), "window.inner_height");

    let f64_inner_height = unwrap!(
        jsvalue_inner_height.as_f64(),
        "jsValue_inner_height.as_f64()"
    );
    let usize_inner_height: usize = unwrap!(f64_inner_height.approx());
    //return
    usize_inner_height
}

/// return window inner width
/// the size of  the visible part of the window
pub fn usize_window_inner_width() -> usize {
    let window = unwrap!(web_sys::window(), "error: web_sys::window");

    let jsvalue_inner_width = unwrap!(window.inner_width(), "window.inner_width");

    let f64_inner_width = unwrap!(
        jsvalue_inner_width.as_f64(),
        "jsValue_inner_width.as_string()"
    );
    let usize_inner_width: usize = unwrap!(f64_inner_width.approx());
    //return
    usize_inner_width
}

/// return window inner width, but maximum 600px
/// the size of  the visible part of the window
pub fn usize_window_inner_width_but_max_600() -> usize {
    let x = usize_window_inner_width();
    if x > 600 {
        //return
        600
    } else {
        //return
        x
    }
}
