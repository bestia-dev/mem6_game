// fetchgamesmetadatamod.rs
//! fetch the names of all games

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::fetchmod;
use crate::gamedatamod;

use unwrap::unwrap;
use web_sys::{Request, RequestInit};
//endregion

///async fetch_response() for gamesmetadata.json
#[allow(clippy::needless_pass_by_value)]
pub fn fetch_games_metadata_request(href: String, vdom_weak: dodrio::VdomWeak) {
    let url_config = format!("{}/content/gamesmetadata.json", href);
    //logmod::debug_write(url_config.as_str());
    let webrequest = create_webrequest(url_config.as_str());
    fetchmod::fetch_response(vdom_weak, &webrequest, &set_game_metadata_from_json);
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
pub fn set_game_metadata_from_json(rrc: &mut RootRenderingComponent, respbody: String) {
    //respbody is json.
    //logmod::debug_write(format!("respbody: {}", respbody).as_str());
    let v: gamedatamod::GamesMetadata = unwrap!(serde_json::from_str(respbody.as_str()));
    rrc.game_data.games_metadata = Some(v.clone());
    //fill the vector
    rrc.game_data.content_folders.clear();
    for x in v.vec_game_metadata {
        rrc.game_data.content_folders.push(x.folder);
    }
}
