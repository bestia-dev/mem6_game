// status_2nd_card_mod.rs
//! code flow from this status

#![allow(clippy::panic)]

// region: use
use crate::*;

use unwrap::unwrap;
use dodrio::{RenderContext, Node, VdomWeak};
use wasm_bindgen::JsCast;
use crate::htmltemplatemod::HtmlTemplating;
use web_sys::{Event, HtmlImageElement};
// endregion

/// on second click
/// The on click event passed by JavaScript executes all the logic
/// and changes only the fields of the Card Grid struct.
/// That struct is the only permanent data storage for later render the virtual dom.
pub fn on_click_2nd_card(
    rrc: &mut RootRenderingComponent,
    vdom: VdomWeak,
    this_click_card_index: usize,
) {
    rrc.game_data.card_index_of_2nd_click = this_click_card_index;
    // flip the card up
    rrc.game_data.get_2nd_card_mut().status = CardStatusCardFace::UpTemporary;
    div_grid_container_mod::play_sound(rrc, this_click_card_index);
    // 2 possible outcomes: 1) Next Player 2) end game/play again
    // that changes: game status,CardStatusCardFace, points or/and player_turn
    // if the cards match, player get one point, but it is the next player turn.
    let is_point = is_point(rrc);
    if is_point {
        update_click_2nd_card_flip_permanently(rrc, is_point);
    }
    let msg_id = ack_msg_mod::prepare_for_ack_msg_waiting(rrc, vdom.clone());
    let msg = websocket_boiler_mod::WsMessageForReceivers {
        msg_sender_ws_uid: rrc.web_data.my_ws_uid,
        msg_receivers_json: rrc.web_data.msg_receivers_json.to_string(),
        msg_data: game_data_mod::WsMessageGameData::MsgClick2ndCard {
            card_index_of_2nd_click: rrc.game_data.card_index_of_2nd_click,
            is_point,
            msg_id,
        },
    };
    ack_msg_mod::send_msg_and_write_in_queue(rrc, &msg, msg_id);
}

/// is all card permanently on
#[allow(clippy::indexing_slicing)]
pub fn is_all_permanently(rrc: &mut RootRenderingComponent) -> bool {
    let mut is_all_permanently = true;
    // the zero element is exceptional, but the iterator uses it
    rrc.game_data.card_grid_data[0].status = CardStatusCardFace::UpPermanently;

    for x in &rrc.game_data.card_grid_data {
        match x.status {
            CardStatusCardFace::UpPermanently => {}
            CardStatusCardFace::Down | CardStatusCardFace::UpTemporary => {
                is_all_permanently = false;
                break;
            }
        }
    }
    // return
    is_all_permanently
}

/// is it a point or not
pub fn is_point(rrc: &RootRenderingComponent) -> bool {
    rrc.game_data.get_1st_card().card_number == rrc.game_data.get_2nd_card().card_number
}

/// msg player click
pub fn on_msg_click_2nd_card(
    rrc: &mut RootRenderingComponent,
    msg_sender_ws_uid: usize,
    card_index_of_2nd_click: usize,
    is_point: bool,
    msg_id: usize,
) {
    ack_msg_mod::send_ack(
        rrc,
        msg_sender_ws_uid,
        msg_id,
        game_data_mod::MsgAckKind::MsgClick2ndCard,
    );
    rrc.game_data.card_index_of_2nd_click = card_index_of_2nd_click;
    update_click_2nd_card_flip_permanently(rrc, is_point);
    update_click_2nd_card_point(rrc, is_point);
}

/// on msg ack player click2nd card
pub fn on_msg_ack_player_click2nd_card(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
    vdom: VdomWeak,
) {
    if ack_msg_mod::remove_ack_msg_from_queue(rrc, player_ws_uid, msg_id) {
        let is_point = is_point(rrc);
        update_click_2nd_card_point(rrc, is_point);
        if is_point {
            // nothing because all happens after the Drink/no drink page
        } else {
            websysmod::debug_write("no");
            status_take_turn_mod::on_click_take_turn(rrc, vdom.clone());
        }
    }
    // TODO: timer if after 3 seconds the ack is not received resend the msg
    // do this 3 times and then hard error
}

/// msg player click
#[allow(clippy::integer_arithmetic)] // points +1 is not going to overflow ever
pub fn update_click_2nd_card_point(rrc: &mut RootRenderingComponent, is_point: bool) {
    if is_point {
        rrc.game_data.game_status = GameStatus::StatusDrink;
        // give points
        rrc.game_data.player_turn_now_mut().points += 1;

        if rrc.game_data.is_my_turn() {
            // drink
            html_template_impl_mod::open_new_local_page("#p06");
        } else {
            // do not drink
            html_template_impl_mod::open_new_local_page("#p07");
        }
    }
}

/// msg player click
#[allow(clippy::integer_arithmetic)] // points +1 is not going to overflow ever
pub fn update_click_2nd_card_flip_permanently(rrc: &mut RootRenderingComponent, is_point: bool) {
    if is_point {
        // the two cards matches. make them permanent FaceUp
        let x1 = rrc.game_data.card_index_of_1st_click;
        let x2 = rrc.game_data.card_index_of_2nd_click;
        unwrap!(rrc.game_data.card_grid_data.get_mut(x1)).status =
            CardStatusCardFace::UpPermanently;
        unwrap!(rrc.game_data.card_grid_data.get_mut(x2)).status =
            CardStatusCardFace::UpPermanently;
    }
}

/// render Play or Wait
pub fn div_click_2nd_card<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
) -> Node<'a> {
    let template_name = if rrc.game_data.is_my_turn() {
        "action_1st_2nd_turn"
    } else {
        "action_1st_2nd_not_turn"
    };
    let html_template = rrc.web_data.get_sub_template(template_name);
    unwrap!(rrc.render_template(cx, &html_template, htmltemplatemod::HtmlOrSvg::Html))
}

/// on click for img in status 2
#[allow(clippy::indexing_slicing)]
pub fn on_click_img_status2nd(root: &mut dyn dodrio::RootRender, vdom: VdomWeak, event: &Event) {
    let rrc = root.unwrap_mut::<RootRenderingComponent>();
    // If the event's target is our image...
    let img = match event
        .target()
        .and_then(|t| t.dyn_into::<HtmlImageElement>().ok())
    {
        None => return,
        // ?? Don't understand what this does. The original was written for Input element.
        Some(input) => input,
    };
    // id attribute of image html element is prefixed with img ex. "img12"
    let this_click_card_index = unwrap!(img.id()[3..].parse::<usize>());
    // click is useful only on facedown cards
    if rrc.game_data.card_grid_data[this_click_card_index]
        .status
        .as_ref()
        == CardStatusCardFace::Down.as_ref()
    {
        status_2nd_card_mod::on_click_2nd_card(rrc, vdom.clone(), this_click_card_index);
        // Finally, re-render the component on the next animation frame.
        vdom.schedule_render();
    } else {
        //only if there is big_img, then make it visible
        websysmod::debug_write("click on img");
        if unwrap!(rrc.game_data.game_config.clone()).big_img == true {
            html_template_impl_mod::visible_big_img(&format!(
                "content/{}/big_img/{}",
                rrc.game_data.game_name,
                unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                    .img_filename
                    .get(
                        unwrap!(rrc.game_data.card_grid_data.get(this_click_card_index))
                            .card_number
                    ))
            ));
        }
    }
}
// div_grid_container() is in div_grid_container_mod.rs
