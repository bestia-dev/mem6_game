// fetch_mod.rs
//! fetch game_config, game metadata, imgs

// region: use
use crate::*;
use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;
use dodrio::VdomWeak;
// endregion

/// async fetch for gameconfig.json and update rrc
pub fn async_fetch_game_config_and_update(rrc: &mut RootRenderingComponent, vdom: VdomWeak) {
    let url_config = format!(
        "{}/content/{}/game_config.json",
        rrc.web_data.href, rrc.game_data.game_name
    );
    spawn_local({
        let vdom_on_next_tick = vdom.clone();
        async move {
            let respbody = websysmod::fetch_response(url_config).await;
            let json = unwrap!(serde_json::from_str(respbody.as_str()));
            // websysmod::debug_write(format!("respbody: {}", respbody).as_str());
            unwrap!(
                vdom_on_next_tick
                    .with_component({
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            rrc.game_data.game_config = json;
                        }
                    })
                    .await
            );
        }
    });
}

/// async fetch for gamesmetadata.json and update rrc
pub fn fetch_games_metadata_and_update(href: &str, vdom: VdomWeak) {
    let url_config = format!("{}/content/gamesmetadata.json", href);
    spawn_local({
        let vdom_on_next_tick = vdom.clone();
        async move {
            // websysmod::debug_write(format!("respbody: {}", respbody).as_str());
            let respbody = websysmod::fetch_response(url_config).await;
            let v: game_data_mod::GamesMetadata = unwrap!(serde_json::from_str(&respbody));
            unwrap!(
                vdom_on_next_tick
                    .with_component({
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
    });
}

/// async fetch for videos.json and update rrc
pub fn fetch_videos_and_update(href: &str, vdom: VdomWeak) {
    let url = format!("{}/content/videos.json", href);
    spawn_local({
        let vdom_on_next_tick = vdom.clone();
        async move {
            let respbody = websysmod::fetch_response(url).await;
            let vid_json: game_data_mod::Videos = unwrap!(serde_json::from_str(&respbody));
            unwrap!(
                vdom_on_next_tick
                    .with_component({
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            // fill the vector
                            rrc.game_data.videos = vid_json.videos;
                        }
                    })
                    .await
            );
        }
    });
}

/// async fetch for audio.json and update rrc
pub fn fetch_audio_and_update(href: &str, vdom: VdomWeak) {
    let url = format!("{}/content/audio.json", href);
    spawn_local({
        let vdom_on_next_tick = vdom.clone();
        async move {
            let respbody = websysmod::fetch_response(url).await;
            let aud_json: game_data_mod::Audio = unwrap!(serde_json::from_str(&respbody));
            unwrap!(
                vdom_on_next_tick
                    .with_component({
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            // fill the vector
                            rrc.game_data.audio = aud_json.audio;
                        }
                    })
                    .await
            );
        }
    });
}

/// fetch all imgs for the cache
#[allow(clippy::needless_pass_by_value)]
pub fn fetch_all_img_for_cache_request(rrc: &mut RootRenderingComponent) {
    let (start_index, end_index) = rrc.game_data.grid_start_end_index();
    for i in start_index..end_index {
        #[allow(clippy::indexing_slicing)]
        // index i is calculated to be inside 0..card_grid_data.len()
        let x = &rrc.game_data.card_grid_data[i];

        let url_img = format!(
            "content/{}/img/{}",
            rrc.game_data.game_name,
            unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                .img_filename
                .get(x.card_number))
        );
        // websysmod::debug_write(&url_img);
        // this is async, so I don't care how much it takes
        // maybe there could be a problem with too much parallel requests
        // from the same browser.
        spawn_local(websysmod::fetch_only(url_img));
    }
}
