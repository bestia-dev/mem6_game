// divfullscreenmod.rs
//! instruction and button for fullscreen

//region: use, const
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::javascriptimportmod;
//use crate::logmod;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
use web_sys;
//endregion

/*
///detect iphone
fn is_iphone() -> bool {
    let window = unwrap!(web_sys::window());
    let navigator = window.navigator();
    let user_agent = unwrap!(navigator.user_agent());
    user_agent.to_ascii_lowercase().contains("iphone")
}
*/

///check the fullscreen_element. works only on android
pub fn is_fullscreen(rrc: &RootRenderingComponent) -> bool {
    if rrc.game_data.is_fullscreen {
        //logmod::debug_write("field is_fullscreen is true");
        true
    } else {
        //logmod::debug_write("is_fullscreen is false");
        let window = unwrap!(web_sys::window());
        //let screen = unwrap!(window.screen());
        let document = unwrap!(window.document());

        //return
        if document.fullscreen_element().is_some() {
            //logmod::debug_write("fullscreen is_some");
            true
        } else {
            //logmod::debug_write("fullscreen is None");
            //if the web app is started from android HomeScreen than
            //it has @media (display-mode: fullscreen)
            let media_query_list = unwrap!(window.match_media("(display-mode: fullscreen)"));
            //logmod::debug_write(&format!("media_query_list: {:?}", media_query_list));
            match media_query_list {
                None => {
                    //logmod::debug_write("media_query_list None");
                    false
                }
                Some(media_query_list) => {
                    //logmod::debug_write("media_query_list Some");
                    //returns true if is started from Home screen
                    media_query_list.matches()
                }
            }
        }
    }
}

///render the div for fullscreen
pub fn div_fullscreen<'b>(is_fullscreen: bool, bump: &'b Bump) -> Vec<Node<'b>> {
    let mut ret_val = vec![dodrio!(bump,
        <div>
        </div>
    )];

    // !is_fullscreen(rrc)
    if !is_fullscreen && javascriptimportmod::is_mobile_device() {
        ret_val.push(dodrio!(bump,
        <div id="div_fullscreen" class="div_clickable"
        onclick={move |root, vdom, _event| {
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            rrc.game_data.is_fullscreen=true;
            javascriptimportmod::do_fullscreen();
            vdom.schedule_render();
            }}>
            <h2 id="h2_fullscreen" >
                {vec![text(
                        bumpalo::format!(in bump, "FullScreen{}", "")
                    .into_bump_str(),
                )]}
            </h2>
        </div>
        ));
        ret_val.push(dodrio!(bump,
        <div>
            <h5>
                {vec![text(
                    bumpalo::format!(in bump, "or choose 'Add to Homescreen' in your browser.{}", "")
                .into_bump_str(),
                )]}
            </h5>
        </div>
        ));
    }
    //return
    ret_val
}
