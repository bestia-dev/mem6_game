// fetchallimgsforcachemod.rs
//! fetch all imgs for cache

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::fetchmod;
use crate::logmod;

use unwrap::unwrap;
use web_sys::{Request, RequestInit};
//endregion

///fetch all imgs for the cache
#[allow(clippy::needless_pass_by_value)]
pub fn fetch_all_img_for_cache_request(
    rrc: &mut RootRenderingComponent,
    vdom_weak: dodrio::VdomWeak,
) {
    for x in &rrc.game_data.card_grid_data {
        if x.card_index_and_id != 0 {
            let url_img = format!(
                "content/{}/img/{}",
                rrc.game_data.game_name,
                unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
                    .img_filename
                    .get(x.card_number_and_img_src))
            );
            logmod::debug_write(url_img.as_str());
            let webrequest = create_webrequest(url_img.as_str());
            //this is async, so I don't care how much it takes
            let v2 = vdom_weak.clone();
            fetchmod::fetch_response(v2, &webrequest, &do_nothing);
        }
    }
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
/// do nothing
pub fn do_nothing(_rrc: &mut RootRenderingComponent, _respbody: String) {}
