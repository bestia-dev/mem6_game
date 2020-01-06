// statusinvitingmod.rs
//! code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use crate::fetchallimgsforcachemod;
use crate::statusgamedatainitmod;

use mem6_common::WsMessage;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///render
pub fn div_inviting<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    dodrio!(bump,
    <div>
        <div>
            <h2 class="h2_user_must_wait">
                {vec![
                    text(bumpalo::format!(in bump, "Players accepted: {}.", rrc.game_data.players.len()-1).into_bump_str()),
                ]}
            </h2>
        </div>
        <div class="div_clickable" onclick={move |root, vdom, _event| {
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            //region: send WsMessage over WebSocket
            statusgamedatainitmod::on_click_start_game(rrc);
            //logmod::debug_write(&format!("MsgStartGame send {}",rrc.game_data.players_ws_uid));
            websocketcommunicationmod::ws_send_msg(
                &rrc.game_data.ws,
                &WsMessage::MsgStartGame {
                    my_ws_uid: rrc.game_data.my_ws_uid,
                    players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                    players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
                    card_grid_data: unwrap!(serde_json::to_string(&rrc.game_data.card_grid_data)),
                    game_config: unwrap!(serde_json::to_string(&rrc.game_data.game_config)),
                },
            );
            let v2 = vdom.clone();
            //async fetch all imgs and put them in service worker cache
            fetchallimgsforcachemod::fetch_all_img_for_cache_request(rrc);
            //endregion
            vdom.schedule_render();
        }}>
            <h2 class="h2_user_can_click">
                {vec![
                    text(bumpalo::format!(in bump, "Start Game?{}", "").into_bump_str()),
                ]}
            </h2>
        </div>
    </div>
    )
}
