// fetchallimgsforcachemod.rs
//! fetch all imgs for cache

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::fetchmod;
use crate::logmod;

use unwrap::unwrap;
use wasm_bindgen_futures::spawn_local;
//endregion

///fetch all imgs for the cache
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
            logmod::debug_write(&url_img);
            //this is async, so I don't care how much it takes
            spawn_local(fetchmod::fetch_only(url_img));
        }
    }
}
