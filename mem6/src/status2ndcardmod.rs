// status2ndcardmod.rs
//! code flow from this status

#![allow(clippy::panic)]

//region: use
use crate::*;

use mem6_common::*;

use unwrap::unwrap;
use dodrio::{
    builder::{ElementBuilder, text},
    bumpalo::{self, Bump},
    Node,
};
use wasm_bindgen::JsCast;
//endregion

/// on second click
/// The on click event passed by JavaScript executes all the logic
/// and changes only the fields of the Card Grid struct.
/// That struct is the only permanent data storage for later render the virtual dom.
pub fn on_click_2nd_card(
    rrc: &mut RootRenderingComponent,
    vdom: &dodrio::VdomWeak,
    this_click_card_index: usize,
) {
    rrc.game_data.card_index_of_second_click = this_click_card_index;
    // flip the card up
    unwrap!(
        rrc.game_data
            .card_grid_data
            .get_mut(rrc.game_data.card_index_of_second_click),
        "error this_click_card_index"
    )
    .status = CardStatusCardFace::UpTemporary;
    divgridcontainermod::play_sound(rrc, this_click_card_index);
    // 2 possible outcomes: 1) Next Player 2) end game/play again
    // that changes: game status,CardStatusCardFace, points or/and player_turn
    // if the cards match, player get one point, but it is the next player turn.
    let is_point = get_is_point(rrc);
    if is_point {
        update_click_2nd_card_flip_permanently(rrc, is_point);
    }
    let msg_id = ackmsgmod::prepare_for_ack_msg_waiting(rrc, vdom);
    let msg = WsMessage::MsgClick2ndCard {
        my_ws_uid: rrc.web_data.my_ws_uid,
        msg_receivers: rrc.web_data.msg_receivers.to_string(),
        card_index_of_second_click: rrc.game_data.card_index_of_second_click,
        is_point,
        msg_id,
    };
    ackmsgmod::send_msg_and_write_in_queue(rrc, &msg, msg_id);
}

/// is all card permanently on
pub fn is_all_permanently(rrc: &mut RootRenderingComponent) -> bool {
    let mut is_all_permanently = true;
    // the zero element is exceptional, but the iterator uses it
    unwrap!(rrc.game_data.card_grid_data.get_mut(0)).status = CardStatusCardFace::UpPermanently;

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
pub fn get_is_point(rrc: &RootRenderingComponent) -> bool {
    unwrap!(rrc
        .game_data
        .card_grid_data
        .get(rrc.game_data.card_index_of_first_click))
    .card_number_and_img_src
        == unwrap!(rrc
            .game_data
            .card_grid_data
            .get(rrc.game_data.card_index_of_second_click))
        .card_number_and_img_src
}
/// msg player click
pub fn on_msg_click_2nd_card(
    rrc: &mut RootRenderingComponent,
    msg_sender_ws_uid: usize,
    card_index_of_second_click: usize,
    is_point: bool,
    msg_id: usize,
) {
    ackmsgmod::send_ack(rrc, msg_sender_ws_uid, msg_id, MsgAckKind::MsgClick2ndCard);
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;
    update_click_2nd_card_flip_permanently(rrc, is_point);
    update_click_2nd_card_point(rrc, is_point);
}

/// on msg ack player click2nd card
pub fn on_msg_ack_player_click2nd_card(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
    vdom: &dodrio::VdomWeak,
) {
    if ackmsgmod::remove_ack_msg_from_queue(rrc, player_ws_uid, msg_id) {
        let is_point = get_is_point(rrc);
        update_click_2nd_card_point(rrc, is_point);
        if is_point {
            // nothing because all happens after the Drink/no drink dialog
        } else {
            websysmod::debug_write("no");
            statustaketurnmod::on_click_take_turn(rrc, vdom);
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
        let player_for_point = unwrap!(rrc.game_data.player_turn.checked_sub(1));
        // give points
        unwrap!(rrc.game_data.players.get_mut(player_for_point)).points += 1;

        if rrc.game_data.my_player_number == player_for_point + 1 {
            // drink
            htmltemplateimplmod::open_new_local_page("#p06");
        } else {
            // do not drink
            htmltemplateimplmod::open_new_local_page("#p07");
        }
    }
}

/// msg player click
#[allow(clippy::integer_arithmetic)] // points +1 is not going to overflow ever
pub fn update_click_2nd_card_flip_permanently(rrc: &mut RootRenderingComponent, is_point: bool) {
    if is_point {
        // the two cards matches. make them permanent FaceUp
        let x1 = rrc.game_data.card_index_of_first_click;
        let x2 = rrc.game_data.card_index_of_second_click;
        unwrap!(rrc.game_data.card_grid_data.get_mut(x1)).status =
            CardStatusCardFace::UpPermanently;
        unwrap!(rrc.game_data.card_grid_data.get_mut(x2)).status =
            CardStatusCardFace::UpPermanently;
    }
}

/// render Play or Wait
#[allow(clippy::integer_arithmetic)]
pub fn div_click_2nd_card<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    if rrc.game_data.my_player_number == rrc.game_data.player_turn {
        ElementBuilder::new(bump, "div")
            .children([ElementBuilder::new(bump, "h2")
                .attr("class", "h2_must_do_something")
                .children([text(
                    bumpalo::format!(in bump,
                        "Play {}",
                        unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname
                    )
                    .into_bump_str(),
                )])
                .finish()])
            .finish()
    } else {
        // return wait for the other player
        ElementBuilder::new(bump, "div")
            .children([ElementBuilder::new(bump, "h2")
                .attr("class", "h2_user_must_wait")
                .children([text(
                    bumpalo::format!(in bump,
                        "Wait for {}",
                        unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname
                    )
                    .into_bump_str(),
                )])
                .finish()])
            .finish()
    }
}

/// on click for img in status 2
pub fn on_click_img_status2nd(
    root: &mut dyn dodrio::RootRender,
    vdom: &dodrio::VdomWeak,
    event: &web_sys::Event,
) {
    let rrc = root.unwrap_mut::<RootRenderingComponent>();
    // If the event's target is our image...
    let img = match event
        .target()
        .and_then(|t| t.dyn_into::<web_sys::HtmlImageElement>().ok())
    {
        None => return,
        // ?? Don't understand what this does. The original was written for Input element.
        Some(input) => input,
    };
    // id attribute of image html element is prefixed with img ex. "img12"
    let this_click_card_index = unwrap!(
        (unwrap!(img.id().get(3..), "error slicing")).parse::<usize>(),
        "error parse img id to usize"
    );
    // click is useful only on facedown cards
    if unwrap!(
        rrc.game_data.card_grid_data.get(this_click_card_index),
        "error this_click_card_index"
    )
    .status
    .as_ref()
        == CardStatusCardFace::Down.as_ref()
    {
        status2ndcardmod::on_click_2nd_card(rrc, vdom, this_click_card_index);
        // Finally, re-render the component on the next animation frame.
        vdom.schedule_render();
    }
}
//div_grid_container() is in divgridcontainermod.rs
