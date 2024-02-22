#![doc(
    html_favicon_url = "https://github.com/bestia-dev/mem6_game/raw/master/webfolder/mem6/images/icons-16.png"
)]
#![doc(
    html_logo_url = "https://github.com/bestia-dev/mem6_game/raw/master/webfolder/mem6/images/icons-192.png"
)]

#![doc=include_str!("../README.md")]

// needed for dodrio ! macro (typed-html)
#![recursion_limit = "512"]
// region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    // variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    // library from dependencies have this clippy warnings. Not my code.
    // Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    // and key information related to it.
    clippy::cargo_common_metadata,
    // Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    // structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    // Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    // version of your dependency, and wildcard dependencies would cause unnecessary 
    // breakage in the ecosystem.
    clippy::wildcard_dependencies,
    // Rust is more idiomatic without return statement
    // Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    // Programmers coming from other languages might prefer the expressiveness of return. 
    // It’s possible to miss the last returning statement because the only difference 
    // is a missing ;. Especially in bigger code with multiple return paths having a 
    // return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    // I have private function inside a function. Self does not work there.
    // Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    // Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    // because then wasm-pack build --target web returns an error: export run not found 
    // Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    // as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    // cannot be inlined across crates. Certain types of crates might intend for most of the 
    // methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    // For these types of crates, enabling this lint might make sense. It allows the crate to 
    // require all exported methods to be #[inline] by default, and then opt out for specific 
    // methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    // Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    // clippy::integer_arithmetic,
    // Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    // Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,
)]
// endregion

// region: mod is used only in lib file. All the rest use use crate
mod ackmsgmod;
mod divgridcontainermod;
mod divplayeractionsmod;
mod fetchmod;
mod gamedatamod;
mod htmltemplateimplmod;
mod rootrenderingcomponentmod;
mod routerimplmod;
mod status1stcardmod;
mod status2ndcardmod;
mod statusdrinkmod;
mod statusgamedatainitmod;
mod statusgameovermod;
mod statusjoinedmod;
mod statusreconnectmod;
mod statustaketurnmod;
mod statuswaitingackmsgmod;
mod storagemod;
mod webdatamod;
mod websocketmod;
// endregion

// this are then used in all the mods if I have there use crate::*;
use crate::gamedatamod::*;
use crate::rootrenderingcomponentmod::RootRenderingComponent;

use dodrio_templating::routermod::Routing;
use dodrio_templating::*;

// use unwrap::unwrap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
#[allow(clippy::shadow_same)]
/// To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    websysmod::debug_write(&format!("wasm app version: {}", env!("CARGO_PKG_VERSION")));

    // Get the container to render the virtual Dom component.
    let div_for_virtual_dom = websysmod::get_element_by_id("div_for_virtual_dom");

    // load from storage or get random (and then save)
    let my_ws_uid = websocketmod::load_or_random_ws_uid();

    let (location_href, href_hash) = websysmod::get_url_and_hash();
    // Construct a new RootRenderingComponent.
    let mut rrc = RootRenderingComponent::new(my_ws_uid);
    rrc.web_data.href = location_href.to_string();
    rrc.web_data.href_hash = href_hash;
    // Mount the component to the `<div id="div_for_virtual_dom">`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    // async fetch_response() for gamesmetadata.json
    let v2 = vdom.weak();
    // TODO: this could be a trait for vdomweak
    fetchmod::fetch_games_metadata_and_update(&location_href, v2);

    let v3 = vdom.weak();
    fetchmod::fetch_videos_and_update(&location_href, v3);

    let v5 = vdom.weak();
    fetchmod::fetch_audio_and_update(&location_href, v5);

    // Start the URL router.
    let v4 = vdom.weak();
    let router = routerimplmod::Router {};
    router.start_router(v4);

    // Run the component forever. Forget to drop the memory.
    vdom.forget();

    Ok(())
}
