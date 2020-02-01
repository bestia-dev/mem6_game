// status2ndcardmod.rs
//! code flow from this status

#![allow(clippy::panic)]

//region: use
use crate::gamedatamod::CardStatusCardFace;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use crate::statustaketurnbeginmod;
use crate::statusgameovermod;
use crate::ackmsgmod;
use crate::logmod;
use crate::divgridcontainermod;
use crate::utilsmod;

use mem6_common::{GameStatus, WsMessage, MsgAckKind};

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///on second click
///The on click event passed by JavaScript executes all the logic
///and changes only the fields of the Card Grid struct.
///That struct is the only permanent data storage for later render the virtual dom.
pub fn on_click_2nd_card(
    rrc: &mut RootRenderingComponent,
    vdom: &dodrio::VdomWeak,
    this_click_card_index: usize,
) {
    rrc.game_data.card_index_of_second_click = this_click_card_index;
    divgridcontainermod::play_sound(rrc, this_click_card_index);
    //3 possible outcomes: 1) same player, 2) Next Player 3) end game/play again
    //that changes: game status,CardStatusCardFace, points or/and player_turn
    //if the cards match, player get one point and continues another turn
    if unwrap!(rrc
        .game_data
        .card_grid_data
        .get(rrc.game_data.card_index_of_first_click))
    .card_number_and_img_src
        == unwrap!(rrc
            .game_data
            .card_grid_data
            .get(rrc.game_data.card_index_of_second_click))
        .card_number_and_img_src
    {
        let msg_id = ackmsgmod::prepare_for_ack_msg_waiting(rrc, vdom);
        let msg = WsMessage::MsgClick2ndCardPoint {
            my_ws_uid: rrc.game_data.my_ws_uid,
            players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
            card_index_of_second_click: rrc.game_data.card_index_of_second_click,
            msg_id,
        };
        ackmsgmod::send_msg_and_write_in_queue(rrc, &msg, msg_id);
        update_click_2nd_card_point(rrc);
    //then wait for ack msg event and then check if is game over
    } else {
        statustaketurnbeginmod::on_click_take_turn_begin(rrc, vdom);
    }
}

///msg player click
pub fn on_msg_click_2nd_card_point(
    rrc: &mut RootRenderingComponent,
    msg_sender_ws_uid: usize,
    card_index_of_second_click: usize,
    msg_id: usize,
) {
    ackmsgmod::send_ack(
        rrc,
        msg_sender_ws_uid,
        msg_id,
        MsgAckKind::MsgClick2ndCardPoint,
    );
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;
    update_click_2nd_card_point(rrc);
}

///on msg ack player click2nd card
pub fn on_msg_ack_player_click2nd_card_point(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
) {
    if ackmsgmod::remove_ack_msg_from_queue(rrc, player_ws_uid, msg_id) {
        logmod::debug_write("update on_msg_ack_player_click2nd_card_point(rrc)");
        update_click_2nd_card_point(rrc);

        //if all the cards are permanenty up, this is the end of the game
        let mut is_all_permanently = true;
        //the zero element is exceptional, but the iterator uses it
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
        if is_all_permanently {
            statusgameovermod::on_msg_game_over(rrc);
            //send message
            websocketcommunicationmod::ws_send_msg(
                &rrc.game_data.ws,
                &WsMessage::MsgGameOver {
                    my_ws_uid: rrc.game_data.my_ws_uid,
                    players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                },
            );
        }
    }
    //TODO: timer if after 3 seconds the ack is not received resend the msg
    //do this 3 times and then hard error
}

///msg player click
#[allow(clippy::integer_arithmetic)] // points +1 is not going to overflow ever
pub fn update_click_2nd_card_point(rrc: &mut RootRenderingComponent) {
    //flip the card up
    unwrap!(
        rrc.game_data
            .card_grid_data
            .get_mut(rrc.game_data.card_index_of_second_click),
        "error this_click_card_index"
    )
    .status = CardStatusCardFace::UpTemporary;

    //give points
    unwrap!(rrc
        .game_data
        .players
        .get_mut(unwrap!(rrc.game_data.player_turn.checked_sub(1))))
    .points += 1;

    // the two cards matches. make them permanent FaceUp
    let x1 = rrc.game_data.card_index_of_first_click;
    let x2 = rrc.game_data.card_index_of_second_click;
    unwrap!(rrc.game_data.card_grid_data.get_mut(x1)).status = CardStatusCardFace::UpPermanently;
    unwrap!(rrc.game_data.card_grid_data.get_mut(x2)).status = CardStatusCardFace::UpPermanently;
    //the same player continues to play
    rrc.game_data.game_status = GameStatus::Status1stCard;
}

///render Play or Wait
#[allow(clippy::integer_arithmetic)]
pub fn div_click_2nd_card<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    if rrc.game_data.my_player_number == rrc.game_data.player_turn {
        dodrio!(bump,
        <div >
            <h2 class="h2_must_do_something">
                {vec![text(bumpalo::format!(in bump, "Play {} {}",
                unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
                utilsmod::ordinal_numbers(rrc.game_data.player_turn)
                ).into_bump_str())]}
            </h2>
        </div>
        )
    } else {
        //return wait for the other player
        dodrio!(bump,
        <h2 class="h2_user_must_wait">
            {vec![text(bumpalo::format!(in bump, "Wait for {} {}",
            unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
            utilsmod::ordinal_numbers(rrc.game_data.player_turn)
            ).into_bump_str())]}
        </h2>
        )
    }
}

//div_grid_container() is in divgridcontainermod.rs
