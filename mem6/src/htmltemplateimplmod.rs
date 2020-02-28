//! htmltemplateimplmod  

use crate::*;
use crate::htmltemplatemod::HtmlTemplating;

use mem6_common::*;
//use qrcode53bytes::*;

use unwrap::unwrap;

use dodrio::{
    Node, RenderContext, RootRender,
    bumpalo::{self},
    builder::{ElementBuilder, text},
};

impl htmltemplatemod::HtmlTemplating for RootRenderingComponent {
    /// html_templating boolean id the next node is rendered or not
    fn call_fn_boolean(&self, fn_name: &str) -> bool {
        websysmod::debug_write(&format!("call_fn_boolean: {}", &fn_name));
        match fn_name {
            "is_first_player" => self.game_data.my_player_number == 1,
            "is_true" => true,
            _ => {
                let x = format!("Error: Unrecognized call_fn_boolean: {}", fn_name);
                websysmod::debug_write(&x);
                true
            }
        }
    }

    /// html_templating functions that return a String
    #[allow(clippy::needless_return, clippy::integer_arithmetic)]
    fn call_fn_string(&self, fn_name: &str) -> String {
        // websysmod::debug_write(&format!("call_fn_string: {}", &fn_name));
        match fn_name {
            "my_nickname" => self.game_data.my_nickname.to_owned(),
            "blink_or_not_nickname" => storagemod::blink_or_not_nickname(self),
            "blink_or_not_group_id" => blink_or_not_group_id(self),
            "my_ws_uid" => format!("{}", self.web_data.my_ws_uid),
            "players_count" => format!("{} ", self.game_data.players.len() - 1),
            "game_name" => self.game_data.game_name.to_string(),
            "group_id" => self.game_data.group_id.to_string(),
            "url_to_join" => format!("bestia.dev/mem6/#p03.{}", self.web_data.my_ws_uid),
            "cargo_pkg_version" => env!("CARGO_PKG_VERSION").to_string(),
            "debug_text" => websysmod::get_debug_text(),
            "gameboard_btn" => {
                // different class depend on status
                "btn".to_owned()
            }
            "card_moniker_first" => {
                return unwrap!(self.game_data.game_config.as_ref()).card_moniker
                    [self.game_data.get_1st_card().card_number]
                    .to_string();
            }
            "card_moniker_second" => {
                return unwrap!(self.game_data.game_config.as_ref()).card_moniker
                    [self.game_data.get_2nd_card().card_number]
                    .to_string();
            }
            "my_points" => {
                return format!("{} ", self.game_data.my_player().points,);
            }
            "player_turn" => {
                return self.game_data.player_turn_now().nickname.to_string();
            }
            _ => {
                let x = format!("Error: Unrecognized call_fn_string: {}", fn_name);
                websysmod::debug_write(&x);
                x
            }
        }
    }

    /// return a closure for the listener.
    fn call_fn_listener(
        &self,
        fn_name: String,
    ) -> Box<dyn Fn(&mut dyn RootRender, dodrio::VdomWeak, web_sys::Event) + 'static> {
        Box::new(move |root, vdom, event| {
            let fn_name = fn_name.clone();
            let fn_name = fn_name.as_str();
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            //websysmod::debug_write(&format!("call_fn_listener: {}", &fn_name));
            match fn_name {
                "nickname_onkeyup" => {
                    storagemod::nickname_onkeyup(rrc, event);
                }
                "group_id_onkeyup" => {
                    storagemod::group_id_onkeyup(rrc, event);
                }
                "open_youtube" => {
                    // randomly choose a link from rrc.videos
                    let num = websysmod::get_random(0, rrc.game_data.videos.len());
                    #[allow(clippy::indexing_slicing)]
                    //cannot panic:the num is 0..video.len
                    websysmod::open_new_tab(&format!(
                        "https://www.youtube.com/watch?v={}",
                        rrc.game_data.videos[num]
                    ));
                }
                "open_menu" => {
                    websysmod::open_new_local_page_push_to_history("#p21");
                }
                "rejoin_resync" => {
                    statusreconnectmod::send_msg_for_resync(rrc);
                }
                "back_to_game" => {
                    let h = unwrap!(websysmod::window().history());
                    let _x = h.back();
                }
                "open_instructions" => {
                    websysmod::open_new_tab("#p08");
                }
                "debug_log" => {
                    websysmod::open_new_tab("#p31");
                }
                "start_a_group_onclick" | "restart_game" => {
                    // send a msg to others to open #p04
                    statusgameovermod::on_msg_play_again(rrc);
                    open_new_local_page("#p02");
                }
                "join_a_group_onclick" => {
                    websysmod::open_new_local_page_push_to_history("#p03");
                }
                "choose_a_game_onclick" => {
                    open_new_local_page("#p05");
                }
                "start_game_onclick" => {
                    statusgamedatainitmod::on_click_start_game(rrc);
                    // async fetch all imgs and put them in service worker cache
                    fetchmod::fetch_all_img_for_cache_request(rrc);
                    vdom.schedule_render();
                    // websysmod::debug_write(&format!("start_game_onclick players: {:?}",rrc.game_data.players));
                    open_new_local_page("#p11");
                }
                "game_type_right_onclick" => {
                    game_type_right_onclick(rrc, vdom);
                }
                "game_type_left_onclick" => {
                    game_type_left_onclick(rrc, vdom);
                }
                "join_group_on_click" => {
                    open_new_local_page("#p04");
                }
                "drink_end" => {
                    // send a msg to end drinking to all players
                    //let audio_element = unwrap!(web_sys::HtmlAudioElement::new_with_src(src));
                    //unwrap!(audio_element.stop());

                    websysmod::debug_write(&format!("MsgDrinkEnd send{}", ""));
                    websocketmod::ws_send_msg(
                        &rrc.web_data.ws,
                        &WsMessage::MsgDrinkEnd {
                            my_ws_uid: rrc.web_data.my_ws_uid,
                            msg_receivers: rrc.web_data.msg_receivers.to_string(),
                        },
                    );
                    // if all the cards are permanently up, this is the end of the game
                    // websysmod::debug_write("if is_all_permanently(rrc)");
                    if status2ndcardmod::is_all_permanently(rrc) {
                        websysmod::debug_write("yes");
                        statusgameovermod::on_msg_game_over(rrc);
                        // send message
                        websocketmod::ws_send_msg(
                            &rrc.web_data.ws,
                            &WsMessage::MsgGameOver {
                                my_ws_uid: rrc.web_data.my_ws_uid,
                                msg_receivers: rrc.web_data.msg_receivers.to_string(),
                            },
                        );
                    } else {
                        statustaketurnmod::on_click_take_turn(rrc, &vdom);
                    }
                    // end the drink dialog
                    open_new_local_page("#p11");
                }
                "p06_load_image" => {
                    //websysmod::debug_write("p06_load_image");
                    statusdrinkmod::play_sound_for_drink(rrc);
                }
                _ => {
                    let x = format!("Error: Unrecognized call_fn_listener: {}", fn_name);
                    websysmod::debug_write(&x);
                }
            }
        })
    }

    /// html_templating functions that return a Node
    #[allow(clippy::needless_return)]
    fn call_fn_node<'a>(&self, cx: &mut RenderContext<'a>, fn_name: &str) -> Node<'a> {
        let bump = cx.bump;
        // websysmod::debug_write(&format!("call_fn_node: {}", &fn_name));
        match fn_name {
            "div_grid_container" => {
                // what is the game_status now?
                // websysmod::debug_write(&format!("game status: {}", self.game_data.game_status));
                let max_grid_size = divgridcontainermod::max_grid_size(self);
                return divgridcontainermod::div_grid_container(self, bump, &max_grid_size);
            }
            "div_player_action" => {
                let node = divplayeractionsmod::div_player_actions_from_game_status(self, cx);
                return node;
            }
            "svg_qrcode" => {
                return svg_qrcode_to_node(self, cx);
            }
            _ => {
                let node = ElementBuilder::new(bump, "h2")
                    .children([text(
                        bumpalo::format!(in bump,
                            "Error: Unrecognized call_fn_node: {}",
                            fn_name
                        )
                        .into_bump_str(),
                    )])
                    .finish();

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
    let link = format!("https://bestia.dev/mem6/#p03.{}", rrc.web_data.my_ws_uid);
    let qr = unwrap!(qrcode53bytes::Qr::new(&link));
    let svg_template = qrcode53bytes::SvgDodrioRenderer::new(222, 222).render(&qr);
    //I added use crate::htmltemplatemod::HtmlTemplating; to make the function prepare_node_from_template in scope.
    unwrap!(rrc.prepare_node_from_template(cx, &svg_template, htmltemplatemod::HtmlOrSvg::Svg))
}

/// the arrow to the right
pub fn game_type_right_onclick(rrc: &mut RootRenderingComponent, vdom: dodrio::VdomWeak) {
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
    fetchmod::async_fetch_game_config_and_update(rrc, vdom);
}

/// left arrow button
pub fn game_type_left_onclick(rrc: &mut RootRenderingComponent, vdom: dodrio::VdomWeak) {
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
    fetchmod::async_fetch_game_config_and_update(rrc, vdom);
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
    let (_old_location_href, old_href_hash) = websysmod::get_url_and_hash();
    if old_href_hash.is_empty() || old_href_hash.starts_with("#p03.") {
        websysmod::open_new_local_page_push_to_history(hash)
    } else {
        let _x = websysmod::window().location().replace(hash);
    }
}

/// if there is already a group_id don't blink
pub fn blink_or_not_group_id(rrc: &RootRenderingComponent) -> String {
    if rrc.game_data.group_id == 0 {
        "blink".to_owned()
    } else {
        "".to_owned()
    }
}
