//! routerimplmod
//! The fill_rrc_local_route() function has specific code to route from the url hash part to a
//! html_template file to fetch. The file name is written to rrc.local_route.  
//! A reference to the function &fill_rrc_local_route() is passed to start_route().
//! This contains only generic code and can be made into a library.

use crate::*;

pub fn fill_rrc_local_route(
    local_route: String,
    rrc: &mut RootRenderingComponent,
    vdom: &dodrio::VdomWeak,
) {
    if local_route == "#p02" {
        let vdom = vdom.clone();
        fetchgmod::async_fetch_game_config_request(rrc, &vdom);
        rrc.web_communication.local_route = "p02_start_a_group.html".to_owned();
    } else if local_route.starts_with("#p03") {
        rrc.game_data.my_player_number = 2;
        if local_route.contains('.') {
            let gr = routermod::get_url_param_in_hash_after_dot(&local_route);
            storagemod::save_group_id_string_to_local_storage(rrc, gr.to_string());
        } else {
            storagemod::load_group_id_string(rrc);
        }
        rrc.web_communication.local_route = "p03_join_a_group.html".to_owned();
    } else if local_route == "#p04" {
        statusjoinedmod::on_load_joined(rrc);
        rrc.web_communication.local_route = "p04_wait_to_start.html".to_owned();
    } else if local_route == "#p05" {
        rrc.web_communication.local_route = "p05_choose_game.html".to_owned();
    } else if local_route == "#p06" {
        rrc.web_communication.local_route = "p06_drink.html".to_owned();
    } else if local_route == "#p07" {
        rrc.web_communication.local_route = "p07_do_not_drink.html".to_owned();
    } else if local_route == "#p08" {
        rrc.web_communication.local_route = "p08_instructions.html".to_owned();
    } else if local_route == "#p11" {
        rrc.web_communication.local_route = "p11_gameboard.html".to_owned();
    } else if local_route == "#p21" {
        rrc.web_communication.local_route = "p21_menu.html".to_owned();
    } else if local_route == "#p31" {
        rrc.web_communication.local_route = "p31_debug_text.html".to_owned();
    } else {
        rrc.web_communication.local_route = "p01_start.html".to_owned();
    }
}