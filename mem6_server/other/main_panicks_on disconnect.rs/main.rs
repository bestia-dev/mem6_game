#![doc(
    html_favicon_url = "https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/icons-16.png"
)]
#![doc(
    html_logo_url = "https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/icons-192.png"
)]
// region: lmake_readme insert "readme.md"
//! # mem6_server
//!
//! version: 19.10.21-20.11  
//!
//! **Html and WebSocket server for the mem6 game**  
//! Primarily made for learning to code Rust for a http + WebSocket server on the same port.  
//! Using Warp for a simple memory game for kids - mem6.  
//! On the IP address on port 8086 listens to http and WebSocket.  
//! Route for http `/` serves static files from folder `/mem6/`.  
//! Route `/mem6ws/` broadcast all WebSocket msg to all connected clients except sender.  
//!
//! ## Google vm
//!
//! One working server is installed on my google vm.  
//! There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8086.
//! Nginx also redirects all http 80 to https 443.  
//! You can play the game here (hosted on google cloud platform):  
//! https://bestia.dev/mem6  

// endregion: lmake_readme insert "readme.md"

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
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::wildcard_dependencies,
    // Rust is more idiomatic without return statement
    clippy::implicit_return,
    // I have private function inside a function. Self does not work there.
    // clippy::use_self,
    // Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    // because then wasm-pack build --target no-modules returns an error: export `run` not found 
    // clippy::missing_inline_in_public_items
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
// endregion

// macro dodrio ! now has warning about a panic?!?
#![allow(clippy::panic)]

// region: use statements
use mem6_common::{WsMessage};

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    net::{SocketAddr, IpAddr, Ipv4Addr},
};
use futures::{future, Future, FutureExt, StreamExt};
use tokio::sync::mpsc;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};
use unwrap::unwrap;
use clap::{App, Arg};
use env_logger::Env;
use log::info;
// endregion

// region: enum, structs, const,...
/// Our status of currently connected users.
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

// endregion

/// main function of the binary
#[tokio::main]
async fn main() {
    // region: env_logger log text to stdout depend on ENV variable

    // pretty_env_logger::init();
    // in Linux : RUST_LOG=info ./mem6_server.exe
    // in Windows I don't know yet.
    // default for env variable info
    let mut builder = env_logger::from_env(Env::default().default_filter_or("info"));
    // nano seconds in the logger
    builder.format_timestamp_nanos();
    builder.init();
    // endregion

    // region: cmdline parameters
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("prm_ip")
                .value_name("ip")
                .default_value("127.0.0.1")
                .help("ip address for listening"),
        )
        .arg(
            Arg::with_name("prm_port")
                .value_name("port")
                .default_value("8086")
                .help("port for listening"),
        )
        .get_matches();

    // from string parameters to strong types
    let fnl_prm_ip = unwrap!(matches.value_of("prm_ip")).to_lowercase();
    let fnl_prm_port = unwrap!(matches.value_of("prm_port")).to_lowercase();

    let local_ip = IpAddr::V4(unwrap!(fnl_prm_ip.parse::<Ipv4Addr>()));
    let local_port = unwrap!(u16::from_str_radix(&fnl_prm_port, 10));
    let local_addr = SocketAddr::new(local_ip, local_port);

    info!(
        "mem6 http server listening on {} and WebSocket on /mem6ws/",
        ansi_term::Colour::Red.paint(local_addr.to_string())
    );
    // endregion

    // Keep track of all connected users, key is usize, value
    // is a WebSocket sender.
    let users = Arc::new(Mutex::new(HashMap::new()));
    // Turn our "state" into a new Filter...
    // let users = warp::any().map(move || users.clone());
    // Clippy recommends this craziness instead of just users.clone()
    let users = warp::any().map(move || {
        Arc::<
            std::sync::Mutex<
                std::collections::HashMap<
                    usize,
                    tokio::sync::mpsc::UnboundedSender<
                        std::result::Result<warp::filters::ws::Message, warp::Error>,
                    >,
                >,
            >,
        >::clone(&users)
    });

    // WebSocket server
    // GET from route /mem6ws/ -> WebSocket upgrade
    let websocket = warp::path("mem6ws")
        // The `ws2()` filter will prepare WebSocket handshake...
        .and(warp::ws())
        .and(users)
        // Match `/mem6ws/url_param` it can be any string.
        .and(warp::path::param::<String>())
        .map(|ws: warp::ws::Ws, users, url_param| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| {
                user_connected(socket, users, url_param).map(|result| unwrap!(result))
            })
        });

    // static file server
    // GET files of route / -> are from folder /mem6/
    let fileserver = warp::fs::dir("./mem6/");

    let routes = fileserver.or(websocket);
    warp::serve(routes).run(local_addr).await;
}

// the url_param is not consumed in this function and Clippy wants
// a reference instead a value
#[allow(clippy::needless_pass_by_value)]
// region: WebSocket callbacks: connect, msg, disconnect
/// new user connects
fn user_connected(
    ws: WebSocket,
    users: Users,
    url_param: String,
) -> impl Future<Output = Result<(), ()>> {
    // the client sends his ws_uid in url_param. it is a random number.
    info!("user_connect() url_param: {}", url_param);
    // convert string to usize
    // hahahahaha syntax 'turbofish' ::<>
    let my_ws_uid = unwrap!(url_param.parse::<usize>());
    // if uid already exists, it is an error
    let mut user_exist = false;
    for (&uid, ..) in unwrap!(users.lock()).iter() {
        if uid == my_ws_uid {
            user_exist = true;
            break;
        }
    }

    if user_exist {
        // disconnect the old user
        info!("user_disconnected for reconnect: {}", my_ws_uid);
        user_disconnected(my_ws_uid, &users);
    }

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the WebSocket...
    let (tx, rx) = mpsc::unbounded_channel();
    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));
    // Save the sender in our list of connected users.
    info!("users.insert: {}", my_ws_uid);
    unwrap!(users.lock()).insert(my_ws_uid, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.
    // Make an extra clone to give to our disconnection handler...
    let users2 = users.clone();
    // Clippy recommends this craziness instead of users.clone()

    user_ws_rx
        // Every time the user sends a message, broadcast it to
        // all other users...
        .for_each(move |msg| {
            user_message(my_ws_uid, unwrap!(msg), &users);
            future::ready(())
        })
        // for_each will keep processing as long as the user stays
        // connected. Once they disconnect, then...
        .then(move |result| {
            user_disconnected(my_ws_uid, &users2);
            future::ok(result)
        })
}

/// on receive WebSocket message
fn user_message(msg_sender_ws_uid: usize, message: Message, users: &Users) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = message.to_str() {
        s
    } else {
        return;
    };

    let new_msg = msg.to_string();
    // info!("msg: {}", new_msg);

    // There are different messages coming from the mem6 wasm app
    // MsgInvite must be broadcasted to all users
    // all others must be forwarded to exactly the other player.

    let msg: WsMessage =
        serde_json::from_str(&new_msg).unwrap_or_else(|_x| WsMessageGameData::MsgDummy {
            dummy: String::from("error"),
        });

    match msg {
        WsMessageGameData::MsgDummy { dummy } => info!("MsgDummy: {}", dummy),
        WsMessageGameData::MsgRequestWsUid {
            my_ws_uid,
            msg_receivers_json,
        } => {
            info!("MsgRequestWsUid: {} {}", my_ws_uid, msg_receivers_json);
            let j = unwrap!(serde_json::to_string(
                &WsMessageGameData::MsgResponseWsUid {
                    your_ws_uid: msg_sender_ws_uid,
                    server_version: env!("CARGO_PKG_VERSION").to_string(),
                }
            ));
            info!("send MsgResponseWsUid: {}", j);
            match unwrap!(unwrap!(users.lock()).get(&msg_sender_ws_uid)).send(Ok(Message::text(j)))
            {
                Ok(()) => (),
                Err(_disconnected) => {}
            }
            // send to other users for reconnect. Do nothing if there is not yet other users.
            send_to_msg_receivers(users, msg_sender_ws_uid, &new_msg, &msg_receivers_json)
        }
        WsMessageGameData::MsgPing { msg_id } => {
            // info!("MsgPing: {}", msg_id);

            let j = unwrap!(serde_json::to_string(&WsMessageGameData::MsgPong {
                msg_id
            }));
            // info!("send MsgPong: {}", j);
            match unwrap!(unwrap!(users.lock()).get(&msg_sender_ws_uid)).send(Ok(Message::text(j)))
            {
                Ok(()) => (),
                Err(_disconnected) => {}
            }
        }
        WsMessageGameData::MsgPong { .. } => {
            unreachable!("mem6_server must not receive MsgPong");
        }
        WsMessageGameData::MsgResponseWsUid { .. } => {
            info!("MsgResponseWsUid: {}", "");
        }
        WsMessageGameData::MsgJoin {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgStartGame {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgClick1stCard {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgClick2ndCard {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgDrinkEnd {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgTakeTurn {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgGameOver {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgPlayAgain {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgAllGameData {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgAck {
            msg_receivers_json, ..
        }
        | WsMessageGameData::MsgAskPlayer1ForResync {
            msg_receivers_json, ..
        } => send_to_msg_receivers(users, msg_sender_ws_uid, &new_msg, &msg_receivers_json),
    }
}

/// New message from this user send to all other players except sender.
fn send_to_msg_receivers(
    users: &Users,
    msg_sender_ws_uid: usize,
    new_msg: &str,
    msg_receivers_json: &str,
) {
    // info!("send_to_msg_receivers: {}", new_msg);

    let vec_msg_receivers: Vec<usize> = unwrap!(serde_json::from_str(msg_receivers_json));

    for (&uid, tx) in unwrap!(users.lock()).iter() {
        let mut is_player;
        is_player = false;
        for &pl_ws_uid in &vec_msg_receivers {
            if pl_ws_uid == uid {
                is_player = true;
            }
        }
        if msg_sender_ws_uid != uid && is_player {
            match tx.send(Ok(Message::text(String::from(new_msg)))) {
                Ok(()) => (),
                Err(_disconnected) => {
                    info!("Err(_disconnected) {}", msg_sender_ws_uid);
                    // The tx is disconnected, our `user_disconnected` code
                    // should be happening in another task, nothing more to
                    // do here.
                }
            }
        }
    }
}

/// disconnect user
fn user_disconnected(my_ws_uid: usize, users: &Users) {
    info!("good bye user: {}", my_ws_uid);

    // Stream closed up, so remove from the user list
    unwrap!(users.lock()).remove(&my_ws_uid);
}
// endregion
