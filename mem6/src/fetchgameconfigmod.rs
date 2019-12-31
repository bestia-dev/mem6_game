// fetchgameconfigmod.rs
//! fetch game_config

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::fetchmod;

use unwrap::unwrap;
use web_sys::{Request, RequestInit};
//endregion

///async fetch_response() for gameconfig.json
pub fn fetch_game_config_request(rrc: &mut RootRenderingComponent, vdom_weak: dodrio::VdomWeak) {
    let url_config = format!(
        "{}/content/{}/game_config.json",
        rrc.game_data.href, rrc.game_data.asked_game_name
    );
    //logmod::debug_write(url_config.as_str());
    let webrequest = create_webrequest(url_config.as_str());
    fetchmod::fetch_response(vdom_weak, &webrequest, &set_game_config_from_json);
}

///create web request from string
pub fn create_webrequest(url: &str) -> web_sys::Request {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let w_webrequest = unwrap!(Request::new_with_str_and_init(url, &opts));

    //logmod::debug_write("let w_webrequest =");
    //return
    w_webrequest
}

#[allow(clippy::needless_pass_by_value)]
/// update a field in the struct
pub fn set_game_config_from_json(rrc: &mut RootRenderingComponent, respbody: String) {
    //respbody is json.
    //logmod::debug_write(format!("respbody: {}", respbody).as_str());
    rrc.game_data.game_config = unwrap!(serde_json::from_str(respbody.as_str()));
}
