//! routerimplmod - A simple `#`-fragment router for dodrio html templating.  
//! Implementation of Router for this mem6 use case with RootRenderingComponent type
//! It routes from short_url (the url hash part) to a
//! html_template file to fetch. The file name is written to rrc.local_route.  
//! Then fetches the file and stores it in rrc.html_template

use crate::*;
use dodrio::VdomWeak;
use unwrap::unwrap;
// I tried to put vdom as a field in Router. But after closures, the vdom
// is not anymore the same and I cannot use the one in Router.
// I must pass vdom as parameter, because it comes from Closures
// and is not anymore the same as self.vdom.
// No other field is really necessary for this struct.

/// empty struct just to implement Router
pub struct Router {}

impl routermod::Routing for Router {
    /// get rrc.local_route
    fn get_rrc_local_route(root: &mut dyn dodrio::RootRender) -> &str {
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        &rrc.web_data.local_route
    }

    /// update local_route with filenames dependent on short_local_route.
    fn update_rrc_local_route(
        local_route: String,
        root: &mut dyn dodrio::RootRender,
        vdom: VdomWeak,
    ) -> String {
        let rrc = root.unwrap_mut::<RootRenderingComponent>();
        if local_route == "#p02" {
            fetchmod::async_fetch_game_config_and_update(rrc, vdom);
            rrc.web_data.local_route = "p02_start_a_group.html".to_owned();
        } else if local_route.starts_with("#p03") {
            rrc.game_data.my_player_number = 2;
            if local_route.contains('.') {
                let gr = routermod::get_url_param_in_hash_after_dot(&local_route);
                storagemod::save_group_id_string_to_local_storage(rrc, gr);
            } else {
                storagemod::load_group_id_string(rrc);
            }
            rrc.web_data.local_route = "p03_join_a_group.html".to_owned();
        } else if local_route == "#p04" {
            statusjoinedmod::on_load_joined(rrc);
            rrc.web_data.local_route = "p04_wait_to_start.html".to_owned();
        } else if local_route == "#p05" {
            rrc.web_data.local_route = "p05_choose_game.html".to_owned();
        } else if local_route == "#p06" {
            rrc.web_data.local_route = "p06_drink.html".to_owned();
        } else if local_route == "#p07" {
            rrc.web_data.local_route = "p07_do_not_drink.html".to_owned();
        } else if local_route == "#p08" {
            rrc.web_data.local_route = "p08_instructions.html".to_owned();
        } else if local_route == "#p11" {
            rrc.web_data.local_route = "p11_gameboard.html".to_owned();
        } else if local_route == "#p21" {
            rrc.web_data.local_route = "p21_menu.html".to_owned();
        } else if local_route == "#p31" {
            rrc.web_data.local_route = "p31_debug_text.html".to_owned();
        } else if local_route == "#p41" {
            rrc.web_data.local_route = "p41_webrtc.html".to_owned();
        } else {
            rrc.web_data.local_route = "p01_start.html".to_owned();
        }
        //return
        rrc.web_data.local_route.to_string()
    }

    /// update html_template
    #[allow(clippy::integer_arithmetic)]
    #[allow(clippy::indexing_slicing)]
    fn update_rrc_html_template(
        resp_body_text: String,
    ) -> Box<dyn Fn(&mut dyn dodrio::RootRender) + 'static> {
        // Callback fired whenever the URL hash fragment changes.
        // Keeps the rrc.web_data.local_route in sync with the `#` fragment.
        Box::new(move |root| {
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            // only the html inside the <body> </body>
            let mut tm = routermod::between_body_tag(&resp_body_text);
            //parse subtemplates <template name="xxx"></template>
            rrc.web_data.vec_html_templates.clear();
            loop {
                let mut exist_template = false;

                let pos1 = tm.find("<template ");
                let del2 = "</template>";
                let pos2 = tm.find(del2);
                if let Some(pos_start) = pos1 {
                    if let Some(pos_end) = pos2 {
                        exist_template = true;
                        //drain - extract a substring and remove it from the original
                        let sub1: String = tm.drain(pos_start..pos_end + del2.len()).collect();

                        let del3 = "name=\"";
                        let pos_name_start = unwrap!(sub1.find(del3));
                        let sub2 = &sub1[pos_name_start + del3.len()..];
                        //websysmod::debug_write(sub2);

                        let pos_name_end = unwrap!(sub2.find('"'));
                        let name = &sub2[0..pos_name_end];
                        //websysmod::debug_write(name);

                        let del5 = '>';
                        let pos_name_end_tag = unwrap!(sub1.find(del5));
                        let pos6 = unwrap!(sub1.find(del2));
                        let sub_template = &sub1[pos_name_end_tag + 1..pos6];
                        //websysmod::debug_write(sub_template);

                        rrc.web_data
                            .vec_html_templates
                            .push((name.to_string(), sub_template.to_string()));
                    }
                }
                if !exist_template {
                    break;
                }
            }
            rrc.web_data.html_template = tm;
        })
    }
}
