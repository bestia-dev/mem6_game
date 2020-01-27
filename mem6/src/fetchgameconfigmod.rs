// fetchgameconfigmod.rs
//! fetch game_config

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::fetchmod;

use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;
//endregion

///async fetch for gameconfig.json
pub fn async_fetch_game_config_request(
    rrc: &mut RootRenderingComponent,
    vdom_weak: dodrio::VdomWeak,
) {
    let url_config = format!(
        "{}/content/{}/game_config.json",
        rrc.game_data.href, rrc.game_data.game_name
    );
    let vdom_weak = vdom_weak.clone();
    spawn_local(set_game_config_from_json(url_config, vdom_weak));
}

//#[allow(clippy::needless_pass_by_value)]
/// update a field in the struct
pub async fn set_game_config_from_json(url_config: String, vdom: dodrio::VdomWeak) {
    let respbody = fetchmod::fetch_response(url_config).await;
    let json = unwrap!(serde_json::from_str(respbody.as_str()));
    //logmod::debug_write(format!("respbody: {}", respbody).as_str());
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
