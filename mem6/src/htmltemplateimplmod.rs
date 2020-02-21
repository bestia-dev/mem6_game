//! htmltemplateimplmod  

use crate::*;
use mem6_common::*;
//use qrcode53bytes::*;

use unwrap::unwrap;
use dodrio::{
    RenderContext,
    bumpalo::{self},
    Node,
    builder::*,
};
use typed_html::dodrio;

const VIDEOS: &[&str] = &[
    "VQdhDw-hE8s",
    "2RT9AzqEfLo",
    "gCDNz8ozT0Y",
    "2RT9AzqEfLo",
    "YgPm3POLqec",
    "9k_ZRyws8Uc",
    "--AvCsh48bk",
    "SCZqBOK8JG4",
    "v7PTqPRFq0w",
    "LeiFF0gvqcc",
    "o3mP3mJDL2k",
    "xwhPlUlkBOc",
    "myS9lF0GrL4",
    "tHjiqz9Q-JQ",
    "JbHo4yrix5o",
    "gfDQPRp8yG4",
    "GnRClOK5R9g",
    "s_rBqCxj3gU",
    "4wAkV_ryyZE",
    "6dxf1VCQi-U",
    "DTxSMX6CfU0",
    "O4hbiab6epE",
    "qViSjEBaDeo",
    "i30sHTaO65U",
    "Yeg4hn8OQko",
    "30CtCHHbhx0",
    "sqfa-6OAJ-A",
    "3CsdVbbdMtU",
    "_Ohi_MJa64Y",
    "N3oCS85HvpY",
    "ICFi4QLlXyo",
    "w7Wx1V9GNcs",
    "YXRENpuksxA",
    "qtcLC0axZmY",
    "lQVeBVipOgI",
    "UzAVaAWLsac",
    "vac5Vq3uas8",
    "CUO8twlE6do",
    "xYfYfB15JmU",
    "A6dDjEKIaMU",
    "1Z1dxe7zkQo",
    "dhO49-agq-E",
    "XnKj8diK5t8",
    "UUZwkSpLyuY",
    "nYLITSwgbMo",
    "4v6bezLr-cU",
    "Wr_dt3mjpaI",
    "iVEdOVSujz8",
    "e3aLX5Avlsc",
    "yTjTIO6zfNo",
    "5xyWlGOeqJY",
    "dBC984kMPuw",
    "kqIe2w13-dY",
    "o2V7qrxjW8k",
    "PjCXl3XZbPU",
    "49VAeeTrOIc",
    "RYQlpDPBb9k",
    "bU_dD3R3fAI",
    "lK5sHyjXfz4",
    "hkHHvb2eDb4",
    "7F-MKVP91XI",
    "3ftey36D12M",
    "0mkD5aptXu8",
    "RXmH_4RKrrc",
    "MeF-e6yPXfo",
    "CHba51EvycY",
    "EY5X9GcOWhk",
    "R9QdgWY01d8",
    "JWA5GTlLCTQ",
    "fvq7PG_6sPk",
    "SX1grPSFjYU",
    "-5u2Nw18z_c",
];

impl htmltemplatemod::HtmlTemplating for RootRenderingComponent {
    // / html_templating boolean id the next node is rendered or not
    fn call_function_boolean(&self, sx: &str) -> bool {
        logmod::debug_write(&format!("call_function_boolean: {}", &sx));
        match sx {
            "is_first_player" => {
                if self.game_data.my_player_number == 1 {
                    true
                } else {
                    false
                }
            }
            _ => {
                let x = format!("Error: Unrecognized call_function_boolean: {}", sx);
                logmod::debug_write(&x);
                true
            }
        }
    }

    // / html_templating functions that return a String
    #[allow(clippy::needless_return, clippy::integer_arithmetic)]
    fn call_function_string(&self, sx: &str) -> String {
        // logmod::debug_write(&format!("call_function_string: {}", &sx));
        match sx {
            "my_nickname" => self.game_data.my_nickname.to_owned(),
            "blink_or_not_nickname" => storagemod::blink_or_not_nickname(self),
            "blink_or_not_group_id" => blink_or_not_group_id(self),
            "my_ws_uid" => format!("{}", self.game_data.my_ws_uid),
            "players_count" => format!("{} ", self.game_data.players.len() - 1),
            "game_name" => self.game_data.game_name.to_string(),
            "group_id" => self.game_data.group_id.to_string(),
            "url_to_join" => format!("bestia.dev/mem6/#p03.{}", self.game_data.my_ws_uid),
            "cargo_pkg_version" => env!("CARGO_PKG_VERSION").to_string(),
            "debug_text" => storagemod::get_debug_text(),
            "gameboard_btn" => {
                // different class depend on status
                "btn".to_owned()
            }
            "card_moniker_first" => {
                return unwrap!(unwrap!(self.game_data.game_config.as_ref())
                    .card_moniker
                    .get(
                        unwrap!(self
                            .game_data
                            .card_grid_data
                            .get(self.game_data.card_index_of_first_click))
                        .card_number_and_img_src
                    ))
                .to_string();
            }
            "card_moniker_second" => {
                return unwrap!(unwrap!(self.game_data.game_config.as_ref())
                    .card_moniker
                    .get(
                        unwrap!(self
                            .game_data
                            .card_grid_data
                            .get(self.game_data.card_index_of_second_click))
                        .card_number_and_img_src
                    ))
                .to_string();
            }
            "my_points" => {
                return format!(
                    "{} ",
                    unwrap!(self
                        .game_data
                        .players
                        .get(self.game_data.my_player_number - 1))
                    .points,
                );
            }
            "player_turn" => {
                return unwrap!(self.game_data.players.get(self.game_data.player_turn - 1))
                    .nickname
                    .to_string();
            }
            _ => {
                let x = format!("Error: Unrecognized call_function_string: {}", sx);
                logmod::debug_write(&x);
                x
            }
        }
    }

    // / html_templating functions for listeners
    // / get a clone of the VdomWeak
    fn call_listener(&mut self, vdom: dodrio::VdomWeak, sx: &str, event: web_sys::Event) {
        // logmod::debug_write(&format!("call_listener: {}", &sx));
        match sx {
            "nickname_onkeyup" => {
                storagemod::nickname_onkeyup(self, event);
            }
            "group_id_onkeyup" => {
                storagemod::group_id_onkeyup(self, event);
            }
            "open_youtube" => {
                // randomly choose a link from VIDEOS
                let num = websysmod::get_random(0, VIDEOS.len());
                open_new_tab(&format!("https://www.youtube.com/watch?v={}", VIDEOS[num]));
            }
            "open_menu" => {
                open_new_local_page_push_to_history("#p21");
            }
            "rejoin_resync" => {
                websocketreconnectmod::send_msg_for_resync(self);
            }
            "back_to_game" => {
                let h = unwrap!(websysmod::window().history());
                let _x = h.back();
            }
            "open_instructions" => {
                open_new_tab("#p08");
            }
            "debug_log" => {
                open_new_tab("#p31");
            }
            "start_a_group_onclick" | "restart_game" => {
                // send a msg to others to open #p04
                statusgameovermod::on_msg_play_again(self);
                open_new_local_page("#p02");
            }
            "join_a_group_onclick" => {
                open_new_local_page_push_to_history("#p03");
            }
            "choose_a_game_onclick" => {
                open_new_local_page("#p05");
            }
            "start_game_onclick" => {
                statusgamedatainitmod::on_click_start_game(self);
                // async fetch all imgs and put them in service worker cache
                fetchgmod::fetch_all_img_for_cache_request(self);
                // endregion
                vdom.schedule_render();
                // logmod::debug_write(&format!("start_game_onclick players: {:?}",self.game_data.players));
                open_new_local_page("#p11");
            }
            "game_type_right_onclick" => {
                game_type_right_onclick(self, &vdom);
            }
            "game_type_left_onclick" => {
                game_type_left_onclick(self, &vdom);
            }
            "join_group_on_click" => {
                open_new_local_page("#p04");
            }
            "drink_end" => {
                // send a msg to end drinking to all players
                logmod::debug_write(&format!("MsgDrinkEnd send{}", ""));
                websocketcommunicationmod::ws_send_msg(
                    &self.game_data.ws,
                    &WsMessage::MsgDrinkEnd {
                        my_ws_uid: self.game_data.my_ws_uid,
                        msg_receivers: self.game_data.msg_receivers.to_string(),
                    },
                );
                // if all the cards are permanently up, this is the end of the game
                // logmod::debug_write("if is_all_permanently(self)");
                if status2ndcardmod::is_all_permanently(self) {
                    logmod::debug_write("yes");
                    statusgameovermod::on_msg_game_over(self);
                    // send message
                    websocketcommunicationmod::ws_send_msg(
                        &self.game_data.ws,
                        &WsMessage::MsgGameOver {
                            my_ws_uid: self.game_data.my_ws_uid,
                            msg_receivers: self.game_data.msg_receivers.to_string(),
                        },
                    );
                } else {
                    statustaketurnmod::on_click_take_turn(self, &vdom);
                }
                // end the drink dialog
                open_new_local_page("#p11");
            }
            _ => {
                let x = format!("Error: Unrecognized call_listener: {}", sx);
                logmod::debug_write(&x);
            }
        }
    }

    // / html_templating functions that return a Node
    #[allow(clippy::needless_return)]
    fn call_function_node<'a>(&self, cx: &mut RenderContext<'a>, sx: &str) -> Node<'a> {
        let bump = cx.bump;
        // logmod::debug_write(&format!("call_function_node: {}", &sx));
        match sx {
            "div_grid_container" => {
                // what is the game_status now?
                // logmod::debug_write(&format!("game status: {}", self.game_data.game_status));
                let max_grid_size = divgridcontainermod::max_grid_size(self);
                return divgridcontainermod::div_grid_container(self, bump, &max_grid_size);
            }
            "div_player_action" => {
                let node = divplayeractionsmod::div_player_actions_from_game_status(self, bump);
                return node;
            }
            "svg_qrcode" => {
                return svg_qrcode_to_node(self, cx);
            }
            _ => {
                let node = dodrio!(bump,
                <h2  >
                    {vec![text(bumpalo::format!(in bump, "Error: Unrecognized call_function_node: {}", sx).into_bump_str())]}
                </h2>
                );
                return node;
            }
        }
    }
}

/// qrcode svg
pub fn svg_qrcode_to_node<'a>(
    rrc: &RootRenderingComponent,
    cx: &mut RenderContext<'a>,
) -> Node<'a> {
    let link = format!("https://bestia.dev/mem6/#p03.{}", rrc.game_data.my_ws_uid);
    let qr = unwrap!(qrcode53bytes::Qr::new(&link));
    let svg_template = qrcode53bytes::SvgDodrioRenderer::new(222, 222).render(&qr);

    unwrap!(htmltemplatemod::get_root_element(
        rrc,
        cx,
        &svg_template,
        htmltemplatemod::HtmlOrSvg::Svg,
    ))
}

/// the arrow to the right
pub fn game_type_right_onclick(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    let gmd = &unwrap!(rrc.game_data.games_metadata.as_ref()).vec_game_metadata;
    let mut last_name = unwrap!(gmd.last()).name.to_string();
    for x in gmd {
        if rrc.game_data.game_name.as_str() == last_name.as_str() {
            rrc.game_data.game_name = x.name.to_string();
            vdom.schedule_render();
            break;
        }
        last_name = x.name.to_string();
    }
    fetchgmod::async_fetch_game_config_request(rrc, vdom);
}

/// left arrow button
pub fn game_type_left_onclick(rrc: &mut RootRenderingComponent, vdom: &dodrio::VdomWeak) {
    let gmd = &unwrap!(rrc.game_data.games_metadata.as_ref()).vec_game_metadata;
    let mut last_name = unwrap!(gmd.first()).name.to_string();
    for x in gmd.iter().rev() {
        if rrc.game_data.game_name.as_str() == last_name.as_str() {
            rrc.game_data.game_name = x.name.to_string();
            vdom.schedule_render();
            break;
        }
        last_name = x.name.to_string();
    }
    fetchgmod::async_fetch_game_config_request(rrc, vdom);
}

/// fn open new local page with #
/// does not push to history
pub fn open_new_local_page(hash: &str) {
    // I want to put the first url in history.
    // These are opened from outside my app and I cannot manage that differently.
    // There are 2 of them:
    // 1. if the players starts without hash
    // 2. if the player scanned the qrcode and opened the p3 with group_id
    // For links opened inside the app, I can call the open with or without history.
    // For example for menu p21 I want to have a back button.
    let (_old_location_href, old_href_hash) = get_url_and_hash();
    if old_href_hash.is_empty() || old_href_hash.starts_with("#p03.") {
        open_new_local_page_push_to_history(hash)
    } else {
        let _x = websysmod::window().location().replace(hash);
    }
}

/// fn open new local page with #
/// and push to history
pub fn open_new_local_page_push_to_history(hash: &str) {
    let _x = websysmod::window().location().assign(hash);
}

/// fn open new tab
pub fn open_new_tab(url: &str) {
    let _w = websysmod::window().open_with_url_and_target(url, "_blank");
}

/// if there is already a group_id don't blink
pub fn blink_or_not_group_id(rrc: &RootRenderingComponent) -> String {
    if rrc.game_data.group_id == 0 {
        "blink".to_owned()
    } else {
        "".to_owned()
    }
}
