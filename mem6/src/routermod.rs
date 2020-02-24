//! routermod - A simple `#`-fragment router for dodrio html templating  
//! This is the generic module. All the specifics for a website are isolated in the  
//! function &fill_rrc_local_route() passed as a parameter to start_router().  
//! The RootRenderingComponent struct must have the fields:  
//! rrc.web_communication.local_route - fill_rrc_local_route() will fills this field.
//!        It will be the name of the html template file to fetch  
//! rrc.web_communication.html_template - a string where the html_template is fetched from the server  

// TODO: how to write this without RootRenderingComponent
// write a Trait that has local route and html_template
use crate::*;

use dodrio::VdomWeak;
//use wasm_bindgen::{prelude::*, JsCast};
//use wasm_bindgen_futures::spawn_local;
use unwrap::unwrap;

pub trait Routing {
    //region: specific code to be implemented
    fn test(&self);
    //endregion: specific code
    fn start_router(&self);
}

//region: generic trait code

/// get the first param after hash in local route after dot
/// example &p03.1234 -> 1234
pub fn get_url_param_in_hash_after_dot(short_local_route: &str) -> &str {
    let mut spl = short_local_route.split('.');
    unwrap!(spl.next());
    unwrap!(spl.next())
}

/// Fetch the html_template and save it in rrc.web_communication.html_template  
/// The async fn for executor spawn_local.  
/// example how to use it in on_click:  
/// ```
/// .on("click", |_root, vdom, _event| {
///     let v2 = vdom;
///     // async executor spawn_local is the recommended for wasm
///     let url = "t1.html".to_owned();
///     // this will change the rrc.web_communication.html_template eventually
///     spawn_local(async_fetch_and_write_to_rrc_html_template(url, v2));
/// })
/// ```
/// async fn cannot be trait fn as of 24.2.2020 cargo version 1.41.0
pub async fn async_fetch_and_write_to_rrc_html_template(url: String, vdom: VdomWeak) {
    websysmod::debug_write(&format!("fetch {}", &url));
    let mut resp_body_text: String = websysmod::async_spwloc_fetch_text(url).await;
    // update values in rrc is async.
    // I can await a fn call or an async block.
    async {
        unwrap!(
            vdom.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    // only the html inside the <body> </body>
                    let pos1 = resp_body_text.find("<body>").unwrap_or(0);
                    let pos2 = resp_body_text.find("</body>").unwrap_or(0);
                    if pos1 != 0 {
                        #[allow(clippy::integer_arithmetic)]
                        {
                            resp_body_text =
                                unwrap!(resp_body_text.get(pos1 + 6..pos2)).to_string();
                        }
                    }
                    // websysmod::debug_write(&format!("body: {}", resp_body_text));
                    rrc.web_communication.html_template = resp_body_text;
                }
            })
            .await
        );
        vdom.schedule_render();
    }
    .await;
}
