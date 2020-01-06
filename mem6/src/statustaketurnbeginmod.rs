// statustaketurnbeginmod.rs
//! code flow for this status

#![allow(clippy::panic)]

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::gamedatamod::{CardStatusCardFace};
use crate::logmod;
use crate::ackmsgmod;
use crate::utilsmod;
use crate::statustaketurnendmod;

use mem6_common::{GameStatus, WsMessage, MsgAckKind};

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///on click
pub fn on_click_take_turn_begin(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    update_on_take_turn_begin(rrc);

    let msg_id = ackmsgmod::prepare_for_ack_msg_waiting(rrc, vdom);
    let msg = &WsMessage::MsgTakeTurnBegin {
        my_ws_uid: rrc.game_data.my_ws_uid,
        players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
        card_index_of_second_click: rrc.game_data.card_index_of_second_click,
        msg_id,
    };
    ackmsgmod::send_msg_and_write_in_queue(rrc, msg, msg_id);
    //then wait for ack msg event
}

///on msg
pub fn on_msg_take_turn_begin(
    rrc: &mut RootRenderingComponent,
    msg_sender_ws_uid: usize,
    card_index_of_second_click: usize,
    msg_id: usize,
) {
    //logmod::debug_write("on_msg_take_turn_begin");
    ackmsgmod::send_ack(rrc, msg_sender_ws_uid, msg_id, MsgAckKind::MsgTakeTurnBegin);
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;

    update_on_take_turn_begin(rrc);
}

///on msg ack
pub fn on_msg_ack_take_turn_begin(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
) {
    if ackmsgmod::remove_ack_msg_from_queue(rrc, player_ws_uid, msg_id) {
        logmod::debug_write("update on_msg_ack_take_turn_begin(rrc)");
        update_on_take_turn_begin(rrc);
    }
}

///update game data
pub fn update_on_take_turn_begin(rrc: &mut RootRenderingComponent) {
    //logmod::debug_write("update_on_take_turn_begin");

    //flip the card up
    unwrap!(
        rrc.game_data
            .card_grid_data
            .get_mut(rrc.game_data.card_index_of_second_click),
        "error this_click_card_index"
    )
    .status = CardStatusCardFace::UpTemporary;
    rrc.game_data.game_status = GameStatus::StatusTakeTurnBegin;
    rrc.check_invalidate_for_all_components();
}

///render div
#[allow(clippy::integer_arithmetic)]
pub fn div_take_turn_begin<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    logmod::debug_write(&format!(
        "div_take_turn_begin: player_turn {}  my_player_number {}",
        &rrc.game_data.player_turn, &rrc.game_data.my_player_number
    ));
    let next_player = if rrc.game_data.player_turn < rrc.game_data.players.len() {
        unwrap!(rrc.game_data.player_turn.checked_add(1))
    } else {
        1
    };
    if rrc.game_data.my_player_number == next_player {
        dodrio!(bump,
        <div class="div_clickable" onclick={move |root, vdom, _event| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    statustaketurnendmod::on_click_take_turn_end(rrc,&vdom);
                }}>
            <h2 class="h2_user_must_click">
                {vec![text(
                    bumpalo::format!(in bump, "{} {}, click here to take your turn",
                        unwrap!(rrc.game_data.players.get(rrc.game_data.my_player_number-1)).nickname,
                        utilsmod::ordinal_numbers(rrc.game_data.my_player_number)
                    )
                        .into_bump_str(),
                )]}
            </h2>
        </div>
        )
    } else {
        //return wait for the other player
        dodrio!(bump,
        <h2 class="h2_user_must_wait">
            {vec![text(bumpalo::format!(in bump, "Wait for {} {}",
            unwrap!(rrc.game_data.players.get(next_player-1)).nickname,
            utilsmod::ordinal_numbers(next_player)
            ).into_bump_str())]}
        </h2>
        )
    }
}
