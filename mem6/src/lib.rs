#![doc(
    html_favicon_url = "https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/icons-16.png"
)]
#![doc(
    html_logo_url = "https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/icons-192.png"
)]
//region: lmake_readme insert "readme.md"
//! # mem6
//!
//! version: 19.10.22-18.40  
//!
//! mem6 is a simple memory game made primarily for learning the Rust programming language and Wasm/WebAssembly with Virtual Dom Dodrio, WebSocket communication and PWA (Progressive Web App).  
//!
//! ## Idea
//!
//! Playing the memory game alone is boring.  
//! Playing it with friends is better.  
//! But if all friends just stare in their smartphones, it is still boring.  
//! What makes memory games (and other board games) entertaining is the company of friends.  
//! There must be many friends around the table watching one another and stealing moves and laughing and screaming at each other.  
//! Today I assume everybody has a decent smartphone. If all friends open the mem6 game and put their smartphones on the center of the table one near the other so that everybody can see them and touch them, this is the closest it gets to a **classic board game**.  
//! All the phones will have a small card grid (ex. 3x3). But the combined card grid from all these phones together is not so small anymore. It is now much more interesting to play for many players.  
//! It can be played with as many friends as there are: 3,4,5,...  
//! More friends - more fun.  
//!
//! ## Rust and Wasm/WebAssembly
//!
//! Rust is a pretty new language created by Mozilla for really low level programming.  
//! It is a step forward from the C language with functionality and features that are best practice today.  
//! It is pretty hard to learn. Some concepts are so different from other languages it makes it
//! hard for beginners. Lifetimes are the strangest and most confusing concept.  
//! The Rust language has been made from the ground up with an ecosystem that makes it productive.  
//! The language and most of the libraries are Open Source. That is good and bad, but mostly good.  
//! Rust is the best language today to compile into Wasm/WebAssembly.  
//! That compiled code works inside a browser directly with the JavaScript engine.  
//! So finally no need for JavaScript to make cross-platform applications inside browsers.  
//! I have a lot of hope here.  
//!
//! ## Virtual DOM
//!
//! Constructing a HTML page with Virtual DOM (vdom) is easier because it is rendered completely on every tick (animation frame).  
//! Sometimes is very complex what should change in the UI when some data changes.  
//! The data can change from many different events and very chaotically (asynchronously).  
//! It is easier to think how to render the complete DOM for a given state of data.  
//! The Rust Dodrio library has ticks, time intervals when it does something. If a rendering is scheduled, it will be done on the next tick. If a rendering is not scheduled I believe nothing happens.  
//! This enables asynchronous changing of data and rendering. They cannot happen theoretically in the
//! same exact moment. So, no data race here.  
//! When GameData change and we know it will affect the DOM, then rendering must be scheduled.  
//! The main component of the Dodrio Virtual Dom is the Root Rendering Component (rrc).  
//! It is the component that renders the complete user interface (HTML).  
//! The root rendering component is easily splitted  into sub-components.  
//!
//! ![subcomponents](https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/subcomponents.png)  
//!
//! Some subcomponents don't need any extra data and can be coded as simple functions.  
//! The subcomponent "players and scores" has its own data. This data is cached from the GameData.  
//! When this data does not match, invalidation is called to cache them.
//! That also schedules the rendering of the subcomponent.  
//! If no data has changed, the cached subcomponent Node is used. This is more efficient and performant.  
//!
//! ## GameData
//!
//! All the game data are in this simple struct.  
//!
//! ## WebSocket communication
//!
//! HTML5 has finally bring a true stateful bidirectional communication.  
//! Most of the programming problems are more easily and effectively solved this way.  
//! The old unidirectional stateless communication is very good for static html pages, but is terrible for any dynamic page. The WebSocket is very rudimental and often the communication breaks for many different reasons. The programmer must deal with it inside the application.  
//! I send simple structs text messages in json format between the players. They are all in the WsMsg enum and therefore easy to use by the server and client.  
//! The WebSocket server is coded especially for this game and recognizes 2 types of msg:
//! TODO: decide on the client what the server will do with the msg.
//!
//! - msg to broadcast to every other player
//! - msg to send only to the actual game players
//!
//! ## WebSockets is not reliable
//!
//! Simple messaging is not reliable. On mobiles it is even worse. There is a lot of possibilities that something goes wrong and the message doesn't reach the destination. The protocol has nothing that can be used to deal with reconnections or lost messages.  
//! That means that I need additional work on the application level - always reply one acknowledgement "ack" message.  
//! Workflow:  
//!
//! - sender sends one message to more players (more ws_uid) with one random number msg_id
//!     push to a vector (msg queue) more items with ws_uid and msg_id
//!     blocks the continuation of the workflow untill receives all ACK from all players
//!
//! - receiver on receive send the ACK aknowledge msg with his ws_uid and msg_id
//!
//! - the sender receives the ACK and removes one item from the vector
//!     if there is no more items for that msg_id, the workflow can continue.
//!     TODO: if after 3 seconds the ACK is not received and error message is shown to the player.
//!
//! This is very similar to a message queue...  
//!
//! ## gRPC, WebRTC datachannel
//!
//! The new shiny protocol gRPC for web communication is great for server-to-server communication. But it is still very limited inside the browser. When it eventually becomes stable I would like to change Websockets for gRPC.  
//! The WebRTC datachannel sounds great for peer-to-peer commnication. Very probably the players will be all on the same wifi network, this solves all latency issues. TODO: in version 6.  
//!
//! ## The game flow
//!
//! In a few words:  
//! Playing player : Status1 - user action - send msg - await for ack msgs - update game data - Status2  
//! Other players: Status1 - receive WsMessage - send ack msg - update game data - Status2  
//!   
//! Before the actual game there is the `invitation and accepting` flow.  
//! It is a little bit different from the game flow. The first player broadcast an invitation msg.  
//! All other players that are in the first status receive that and are asked if they accept.  
//! When the user Accepts it sends a msg to the first player.  
//! The first player waits to receive msgs from all other users.  
//! After that he starts the game. This calculates the game_data and send this init data to all other players.  
//!
//! | Game Status1         | Render               | User action           | GameStatus2 p.p. | Sends Msg       | On rcv Msg o.p.       | GameStatus2 o.p. |
//! | -------------------- | -------------------- | --------------------- | ---------------- | --------------  | -------------------   | --------------   |
//! | p.p. StatusStartPage | div_start_page       | on_click_invite       | StatusInviting   | MsgInvite       | on_msg_invite         | StatusInvited    |
//! | o.p. StatusInvited   | div_invited          | on_click_accept       | StatusAccepted   | MsgAccept       | on_msg_accept         | -                |
//! | o.p. StatusAccepted  | div_invite_accepted  |                       |                  |                 |                       | -                |
//! | p.p. StatusInviting  | div_inviting         | on_click_start_game   | Status1stCard    | MsgStartGame    | on_msg_start_game     | Status1stCard    |
//!
//! This starts the game flow, that repeats until the game is over.  
//!   
//! In one moment the game is in a one Game Status for all players.  
//! One player is the playing player and all others are awaiting.  
//! The active user then makes an action on the GUI.
//! This action will eventually change the GameData and the GameStatus. But before that there is communication.  
//! A message is sent to other players so they can also change their local GameData and GameStatus.  
//! Because of unreliable networks ther must be an acknowledge ack msg to confirm, that the msg is received to continue the game.  
//! The rendering is scheduled and it will happen shortly (async).  
//!
//! | Game Status1      | Render                     | User action                    | Condition                     | GameStatus2 p.p.    | Sends Msg            | On rcv Msg o.p.             | GameStatus2 o.p.               |
//! | ----------------  | -------------------------- | ------------------------------ | ----------------------------- | ----------------    | ----------------     | --------------------------  | ----------------------------   |
//! | Status1stCard     | div_grid_container         | on_click_1st_card()            | -                             | Status2ndCard       | MsgClick1stCard      | on_msg_click_1st_card       | Status2ndCard                  |
//! | Status2ndCard     | div_grid_container         | on_click_2nd_card()            | If cards match                | Status1stCard       | MsgClick2ndCardPoint | on_msg_click_2nd_card_point | Status1stCard                  |
//! | -                 | -                          | continues on ack msgs received | if all cards permanently up   | StatusGameOver      | MsgGameOver          | on_msg_game_over            | StatusGameOver                 |
//! | Status2ndCard     | div_grid_container         | on_click_take_turn_begin       | else cards don't match        | StatusTakeTurnBegin | MsgTakeTurnBegin     | on_msg_take_turn_begin      | MsgTakeTurnBegin               |
//! | MsgTakeTurnBegin  | div_take_turn_begin        | on_click_take_turn_end         | -                             | Status1stCard       | MsgTakeTurnEnd       | on_msg_take_turn_end        | Status1stCard, the next player |
//! | StatusGameOver    | div_game_over              | window.location().reload()     | -                             | -                   | -                    | -                           | -                              |
//! |  |  |  |  |  |  |  |  |
//!
//! p.p. = playing player,   o.p. = other players,  rrc = rrc, rcv = receive
//!
//! 1. Some actions can have different results. For example the condition if card match or if card don’t match.  
//! 2. one action must be only for one status1. This action changes Status for this player and sends Msg to other players.  
//! 3. on receive msg can produce only one status2.  
//!
//! ## Futures and Promises, Rust and JavaScript
//!
//! JavaScript is all asynchronous. Wasm is nothing else then a shortcut to the JavaScript engine.  
//! So everything is asynchronous too. This is pretty hard to grasp. Everything is Promises and Futures.  
//! There is a constant jumping from thinking in Rust to thinking is JavaScript and back. That is pretty confusing.  
//! JavaScript does not have a good idea of Rust datatypes. All there is is a generic JSValue type.  
//! The library `wasm-bindgen` has made a fantastic job of giving Rust the ability to call
//! anything JavaScript can call, but the way of doing it is sometimes cumbersome.  
//!
//! ## Typed html
//!
//! Writing html inside Rust code is much easier with the macro `html!` from the `crate typed-html`  
//! <https://github.com/bodil/typed-html>  
//! It has also a macro `dodrio!` created exclusively for the dodrio vdom.  
//! Everything is done in compile time, so the runtime is nothing slower.  
//! TODO: what if an attribute is not covered by the macro. Can I add it later?
//!
//! ## Browser console
//!
//! At least in modern browsers (Firefox and Chrome) we have the developer tools F12 and there is a
//! console we can output to. So we can debug what is going on with our Wasm program.
//! But not on smartphones !! I save the eroor and log messages in sessionStorage and this is displayed on the screen.  
//!
//! ## Safari on iOS and FullScreen
//!
//! Apple is very restrictive and does not allow fullscreen Safari on iPhones.  
//! The workaround is to `Add to Homescreen` the webapp.  
//!
//! ## PWA (Progressive Web App)
//!
//! On both android and iPhone is possible to use PWA.  
//! To be 100% PWA it must be secured with TLS and must have a service worker.  
//! I added also the PWA manifest and png images for icons and now the game is a full PWA.  
//!
//! **Very important :**
//! On Android Chrome to `Clear & reset` the cached data of the website you must click on the icon of the URL address (the lock) and choose `Site Settings`.  
//! Sometimes even that does not work. Than I go in the Menu to Settings - Privacy - Clear browser data and delete all. Very aggressive, but the only thing that works.  
//!
//! ## Modules
//!
//! Rust code is splitted into modules. They are not exactly like classes, but can be similar.  
//! Rust has much more freedom to group code in different ways. So that is best suits the problem.  
//! I splitted the rendering into sub-components.  
//! And then I splitted the User Actions by the Status1 to easy follow the flow of the game.  
//!
//! ## Clippy
//!
//! Clippy is very useful to teach us how to program Rust in a better way.  
//! These are not syntax errors, but hints how to do it in a more Rusty way (idiomatic).  
//! Some lints are problematic and they are explicitly allowed here.
//!
//! ## font-size
//!
//! Browsers have 2 types of zoom:
//!
//! - zoom everything proportionally (I like it)
//! - zoom only the text (this breaks the layout completely)
//!
//! When the font-size in android is increased (accessibility) it applies somehow also to the browser rendering.  
//! I have tried many different things, but it looks this cannot be overridden from the css or javascript. Only the user can change this setting in his phone.  

//endregion: lmake_readme insert "readme.md"

//needed for dodrio! macro (typed-html)
#![recursion_limit = "512"]
//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    //Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    //and key information related to it.
    clippy::cargo_common_metadata,
    //Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    //structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    //Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    //version of your dependency, and wildcard dependencies would cause unnecessary 
    //breakage in the ecosystem.
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    //Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    //Programmers coming from other languages might prefer the expressiveness of return. 
    //It’s possible to miss the last returning statement because the only difference 
    //is a missing ;. Especially in bigger code with multiple return paths having a 
    //return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export run not found 
    //Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    //as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    //cannot be inlined across crates. Certain types of crates might intend for most of the 
    //methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    //For these types of crates, enabling this lint might make sense. It allows the crate to 
    //require all exported methods to be #[inline] by default, and then opt out for specific 
    //methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    //Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    //clippy::integer_arithmetic,
    //Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    //Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,
)]
//endregion

//region: mod is used only in lib file. All the rest use use crate
mod ackmsgmod;
mod divcardmonikermod;
mod divfordebuggingmod;
mod divfullscreenmod;
mod divgametitlemod;
mod divgridcontainermod;
mod divplayeractionsmod;
mod divplayersandscoresmod;
mod divrulesanddescriptionmod;
mod fetchmod;
mod fetchgamesmetadatamod;
mod fetchgameconfigmod;
mod fetchallimgsforcachemod;
mod gamedatamod;
mod javascriptimportmod;
mod divnicknamemod;
mod logmod;
mod page01nicknamemod;
mod page02groupmod;
mod page03gamemod;
mod page04instructionsmod;
mod page05errormod;
mod rootrenderingcomponentmod;
mod sessionstoragemod;
mod statusgamedatainitmod;
mod statusstartpagemod;
mod statusinvitedmod;
mod statusinvitingmod;
mod statusgameovermod;
mod status1stcardmod;
mod status2ndcardmod;
mod statustaketurnbeginmod;
mod statustaketurnendmod;
mod statuswaitingackmsgmod;
mod utilsmod;
mod websocketcommunicationmod;
mod websocketreconnectmod;
//endregion

//region: use statements
use unwrap::unwrap;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::Rng;
use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
use wasm_bindgen::prelude::*;
use conv::{ConvUtil};
use conv::{ConvAsUtil};
//endregion

#[wasm_bindgen(start)]
#[allow(clippy::shadow_same)]
///To start the Wasm application, wasm_bindgen runs this functions
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's container to render the virtual Dom component.
    let window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let div_for_virtual_dom = unwrap!(document.get_element_by_id("div_for_virtual_dom"));

    //font-sie for window less then 300 and more then 600 does not need to be calculated
    let window_width =
        unwrap!(unwrap!(JsValue::as_f64(&unwrap!(window.inner_width()))).approx_as::<usize>());
    let mut str_px = "16px".to_owned();
    if window_width > 600 {
        //fixed size
        str_px = "32px".to_owned();
    } else if window_width < 300 {
        //size relative to viewport
        str_px = "3vw".to_owned();
    } else {
        //manually calculated size
        //font-size is a nightmare. Cannot find a way to make fix font-size with css
        //on mobile browsers with bigger font for accessibility enabled.
        //I will try to get the width of a string of known length
        //in the browser and then calculate what a font-size should be.
        let test_text_width = unwrap!(document.get_element_by_id("forfontwidth"));
        let html_element_test_width =
            unwrap!(test_text_width.dyn_into::<web_sys::HtmlSpanElement>());
        let text_width = html_element_test_width.offset_width();
        let body = unwrap!(document.body());
        let body_width = body.client_width();
        logmod::debug_write(&format!(
            "width text {:?}px as 16px inside body {:?}px window {:?}px",
            text_width, body_width, window_width
        ));
        //just change factor if fonts are too big or too small
        //0.7 is 70% of the body width I want to have for this text example.
        let factor = (body_width as f64 * 0.7) / text_width as f64;
        let px = 16.0 * factor;
        str_px = format!("{:.2}px", px);
        logmod::debug_write(&format!("calculated px  {:?} {}", px, &str_px));
    }
    //the root element <html> for css style
    let elem_doc_el = unwrap!(document.document_element());
    let html_html_element_doc_elem = unwrap!(elem_doc_el.dyn_into::<web_sys::HtmlHtmlElement>());
    html_html_element_doc_elem
        .style()
        .set_property("font-size", &str_px);

    //my_ws_uid is random generated on the client and sent to
    //WebSocket server with an url param. It is saved locally to allow reconnection
    //if there are connection problems.
    let mut my_ws_uid: usize = sessionstoragemod::load_my_ws_uid();
    if my_ws_uid == 0 {
        let mut rng = SmallRng::from_entropy();
        my_ws_uid = rng.gen_range(1, 9999);
        sessionstoragemod::save_my_ws_uid(my_ws_uid);
    }
    //from now on, I don't want it mutable (variable shadowing).
    let my_ws_uid = my_ws_uid;

    //find out URL
    let mut location_href = unwrap!(window.location().href(), "href not known");
    //without /index.html
    location_href = location_href.to_lowercase().replace("index.html", "");
    logmod::debug_write(&format!("location_href: {}", &location_href));

    //WebSocket connection
    let players_ws_uid = "[]".to_string(); //empty vector in json
    let ws = websocketcommunicationmod::setup_ws_connection(
        location_href.clone(),
        my_ws_uid,
        players_ws_uid,
    );
    //I don't know why is needed to clone the WebSocket connection
    let ws_c = ws.clone();

    // Construct a new RootRenderingComponent.
    //I added ws_c so that I can send messages on WebSocket

    let mut rrc = rootrenderingcomponentmod::RootRenderingComponent::new(ws_c, my_ws_uid);
    rrc.game_data.href = location_href.to_string();
    rrc.game_data.is_fullscreen = divfullscreenmod::is_fullscreen(&rrc);
    // Mount the component to the `<div id="div_for_virtual_dom">`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    websocketcommunicationmod::setup_all_ws_events(&ws, vdom.weak());

    //async fetch_response() for gamesmetadata.json
    let v2 = vdom.weak();
    fetchgamesmetadatamod::fetch_games_metadata_request(location_href, v2);

    // Run the component forever. Forget to drop the memory.
    vdom.forget();

    Ok(())
}
