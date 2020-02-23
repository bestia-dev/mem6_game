// fetchgmod.rs
//! fetch game_config, game metadata, imgs

//region: use
use crate::*;
use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;
//endregion

//region: game_config
/// async fetch for gameconfig.json
pub fn async_fetch_game_config_request(
    rrc: &mut RootRenderingComponent,
    vdom_weak: &dodrio::VdomWeak,
) {
    let url_config = format!(
        "{}/content/{}/game_config.json",
        rrc.web_communication.href, rrc.game_data.game_name
    );
    let vdom_weak = vdom_weak.clone();
    spawn_local(set_game_config_from_json(url_config, vdom_weak));
}

//#[allow(clippy::needless_pass_by_value)]
/// update a field in the struct
pub async fn set_game_config_from_json(url_config: String, vdom: dodrio::VdomWeak) {
    let respbody = websysmod::fetch_response(url_config).await;
    let json = unwrap!(serde_json::from_str(respbody.as_str()));
    // websysmod::debug_write(format!("respbody: {}", respbody).as_str());
    unwrap!(
        vdom.with_component({
            move |root| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                rrc.game_data.game_config = json;
            }
        })
        .await
    );
}
//endregion: game_config

//region: game_metadata
/// async fetch for gamesmetadata.json
#[allow(clippy::needless_pass_by_value)]
pub fn fetch_games_metadata_request(href: String, vdom_weak: dodrio::VdomWeak) {
    let url_config = format!("{}/content/gamesmetadata.json", href);
    spawn_local(set_game_metadata_from_json(url_config, vdom_weak));
}

/// update a field in the struct
pub async fn set_game_metadata_from_json(url_config: String, vdom: dodrio::VdomWeak) {
    // websysmod::debug_write(format!("respbody: {}", respbody).as_str());
    let respbody = websysmod::fetch_response(url_config).await;
    let v: gamedatamod::GamesMetadata = unwrap!(serde_json::from_str(&respbody));
    unwrap!(
        vdom.with_component({
            move |root| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                // fill the vector
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
//endregion: game_metadata

/// fetch all imgs for the cache
#[allow(clippy::needless_pass_by_value)]
pub fn fetch_all_img_for_cache_request(rrc: &mut RootRenderingComponent) {
    for x in &rrc.game_data.card_grid_data {
        if x.card_index_and_id != 0 {
            let url_img = format!(
                "content/{}/img/{}",
                rrc.game_data.game_name,
                unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                    .img_filename
                    .get(x.card_number_and_img_src))
            );
            // websysmod::debug_write(&url_img);
            // this is async, so I don't care how much it takes
            spawn_local(websysmod::fetch_only(url_img));
        }
    }
}
