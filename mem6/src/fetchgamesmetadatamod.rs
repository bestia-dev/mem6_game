// fetchgamesmetadatamod.rs
//! fetch the names of all games

//region: use
use crate::*;

use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;
//endregion

///async fetch for gamesmetadata.json
#[allow(clippy::needless_pass_by_value)]
pub fn fetch_games_metadata_request(href: String, vdom_weak: dodrio::VdomWeak) {
    let url_config = format!("{}/content/gamesmetadata.json", href);
    spawn_local(set_game_metadata_from_json(url_config, vdom_weak));
}

/// update a field in the struct
pub async fn set_game_metadata_from_json(url_config: String, vdom: dodrio::VdomWeak) {
    //logmod::debug_write(format!("respbody: {}", respbody).as_str());
    let respbody = fetchmod::fetch_response(url_config).await;
    let v: gamedatamod::GamesMetadata = unwrap!(serde_json::from_str(&respbody));
    unwrap!(
        vdom.with_component({
            move |root| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                //fill the vector
                rrc.game_data.content_folders.clear();
                for x in &v.vec_game_metadata {
                    rrc.game_data.content_folders.push(x.folder.clone());
                }
                rrc.game_data.games_metadata = Some(v);
            }
        })
        .await
    );
}
