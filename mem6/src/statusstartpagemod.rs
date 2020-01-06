// statusstartpagemod.rs
//! code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use crate::fetchgameconfigmod;
use crate::gamedatamod;

use mem6_common::{GameStatus, Player, WsMessage};

//use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
//endregion

///render invite ask begin, ask to play for multiple contents/folders
pub fn div_start_page<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    let mut vec_of_nodes = Vec::new();
    //I don't know how to solve the lifetime problems. So I just clone the small data.
    let ff = rrc.game_data.content_folders.clone();
    for folder_name in ff {
        let folder_name_clone2 = folder_name.clone();
        vec_of_nodes.push(dodrio!(bump,
        <div class="div_clickable" onclick={move |root, vdom, _event| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                let v2= vdom.clone();
                on_click_invite(rrc, &folder_name,v2);

                vdom.schedule_render();
                }}>
            <h2 class="h2_user_can_click">
                {vec![text(
                //show Ask Player2 to Play!
                bumpalo::format!(in bump, "{}", folder_name_clone2)
                    .into_bump_str(),
                )]}
            </h2>
        </div>
        ));
    }
    dodrio!(bump,
    <div>
        <h4>
           {vec![text(
                bumpalo::format!(in bump, "Wait for invitation or invite for {}", "")
                    .into_bump_str(),
                )]}
        </h4>
        {vec_of_nodes}
    </div>
    )
}

/// on click updates some data and sends msgs
/// msgs will be asynchronously received and processed
pub fn on_click_invite(
    rrc: &mut RootRenderingComponent,
    folder_name: &str,
    vdom_weak: dodrio::VdomWeak,
) {
    rrc.game_data.my_player_number = 1;
    rrc.game_data.players.clear();
    rrc.game_data.players.push(Player {
        ws_uid: rrc.game_data.my_ws_uid,
        nickname: rrc.game_data.my_nickname.clone(),
        points: 0,
    });
    rrc.game_data.game_status = GameStatus::StatusInviting;
    rrc.game_data.asked_game_name = folder_name.to_string();

    //async fetch_response() for gameconfig.json
    fetchgameconfigmod::async_fetch_game_config_request(rrc, vdom_weak);
    //send the msg MsgInvite
    //logmod::debug_write(&format!("MsgInvite send {}", rrc.game_data.my_ws_uid));
    websocketcommunicationmod::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::MsgInvite {
            my_ws_uid: rrc.game_data.my_ws_uid,
            my_nickname: rrc.game_data.my_nickname.clone(),
            asked_game_name: folder_name.to_string(),
        },
    );
}

///msg invite
pub fn on_msg_invite(
    rrc: &mut RootRenderingComponent,
    his_ws_uid: usize,
    his_nickname: String,
    asked_game_name: String,
) {
    //logmod::debug_write(&format!("on_msg_invite {}", his_ws_uid));
    rrc.reset();
    rrc.game_data.game_status = GameStatus::StatusInvited;
    //the first player is the initiator
    rrc.game_data.players.push(Player {
        ws_uid: his_ws_uid,
        nickname: his_nickname,
        points: 0,
    });
    rrc.game_data.players.push(Player {
        ws_uid: rrc.game_data.my_ws_uid,
        nickname: rrc.game_data.my_nickname.clone(),
        points: 0,
    });
    rrc.game_data.my_player_number = 2; //temporary number
    rrc.game_data.asked_game_name = asked_game_name;
    //always generate the json string for the server
    rrc.game_data.players_ws_uid = gamedatamod::prepare_players_ws_uid(&rrc.game_data.players);
}
