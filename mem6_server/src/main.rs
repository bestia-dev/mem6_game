#![doc(
    html_favicon_url = "https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/icons-16.png"
)]
#![doc(
    html_logo_url = "https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/icons-192.png"
)]
// region: lmake_readme insert "readme.md"
//! # mem6_server
//!
//! version: 2020.221.1018  
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
//! <https://bestia.dev/mem6>  
//!
//! ## new version of Warp
//!
//! The new version looks nice, but I had the problem when a user disconnects the websocket without handshake. It happens only on Android Chrome.  

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

// region: use statements
use mem6_common::*;

use unwrap::unwrap;
use clap::{App, Arg};
use env_logger::Env;
use futures::{sync::mpsc, Future, Stream};
use std::{
    collections::HashMap,
    net::{SocketAddr, IpAddr, Ipv4Addr},
    sync::{Arc, Mutex},
};
use warp::{
    ws::{Message, WebSocket},
    Filter,
};
use log::info;
use serde_derive::{Serialize, Deserialize};
// endregion

// region: enum, structs, const,...
/// Our status of currently connected ws_users.
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type WsUsers = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

/// message for receivers. The original declaration is in the wasm module
/// this here is only a copy that is needed for the server
/// Here, I ignore the msg_data completely.
#[derive(Serialize, Deserialize, Clone)]
pub struct WsMessageForReceivers {
    /// ws client instance unique id. To not listen the echo to yourself.
    pub msg_sender_ws_uid: usize,
    /// only the players that reconnected
    pub msg_receivers_json: String,
    // msg data
    // pub msg_data: WsMessageData,
}
// endregion

/// main function of the binary
fn main() {
    // region: env_logger log text to stdout depend on ENV variable
    // in Linux : RUST_LOG=info ./mem6_server.exe
    // in Windows I don't know yet.
    // default for env variable info
    let mut builder = env_logger::from_env(Env::default().default_filter_or("info"));
    // nanoseconds in the logger
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
    let fnl_prm_ip = matches
        .value_of("prm_ip")
        .expect("error on prm_ip")
        .to_lowercase();
    let fnl_prm_port = matches
        .value_of("prm_port")
        .expect("error on prm_port")
        .to_lowercase();

    let local_ip = IpAddr::V4(fnl_prm_ip.parse::<Ipv4Addr>().expect("not an ip address"));
    let local_port = u16::from_str_radix(&fnl_prm_port, 10).expect("not a number");
    let local_addr = SocketAddr::new(local_ip, local_port);

    info!(
        "mem6 http server listening on {} and WebSocket on /mem6ws/",
        ansi_term::Colour::Red.paint(local_addr.to_string())
    );
    // endregion

    // Keep track of all connected ws_users, key is usize, value
    // is a WebSocket sender.
    let ws_users = Arc::new(Mutex::new(HashMap::new()));
    // Turn our "state" into a new Filter...
    // let ws_users = warp::any().map(move || ws_users.clone());
    // Clippy recommends this craziness instead of just ws_users.clone()
    let ws_users = warp::any().map(move || {
        Arc::<
            std::sync::Mutex<
                std::collections::HashMap<
                    usize,
                    futures::sync::mpsc::UnboundedSender<warp::ws::Message>,
                >,
            >,
        >::clone(&ws_users)
    });

    // WebSocket server
    // GET from route /mem6ws/ -> WebSocket upgrade
    let websocket = warp::path("mem6ws")
        // The `ws2()` filter will prepare WebSocket handshake...
        .and(warp::ws2())
        .and(ws_users)
        // Match `/mem6ws/url_param` it can be any string.
        .and(warp::path::param::<String>())
        .map(|ws: warp::ws::Ws2, ws_users, url_param| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket, ws_users, url_param))
        });

    // static file server
    // GET files of route / -> are from folder /mem6/
    let fileserver = warp::fs::dir("./mem6/");

    let routes = fileserver.or(websocket);
    warp::serve(routes).run(local_addr);
}

// the url_param is not consumed in this function and Clippy wants a reference instead a value
#[allow(clippy::needless_pass_by_value)]
// region: WebSocket callbacks: connect, msg, disconnect
/// new user connects
fn user_connected(
    ws: WebSocket,
    ws_users: WsUsers,
    url_param: String,
) -> impl Future<Item = (), Error = ()> {
    // the client sends his ws_uid in url_param. it is a random number.
    info!("user_connect() url_param: {}", url_param);
    // convert string to usize
    // hahahahaha syntax 'turbofish' ::<>
    let my_ws_uid = unwrap!(url_param.parse::<usize>());
    // if uid already exists, it is an error
    let mut user_exist = false;
    for (&uid, ..) in ws_users.lock().expect("error ws_users.lock()").iter() {
        if uid == my_ws_uid {
            user_exist = true;
            break;
        }
    }

    if user_exist {
        // disconnect the old user
        info!("user_disconnected for reconnect: {}", my_ws_uid);
        user_disconnected(my_ws_uid, &ws_users);
    }

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the WebSocket...
    let (tx, rx) = mpsc::unbounded();
    warp::spawn(
        rx.map_err(|()| -> warp::Error { unreachable!("unbounded rx never errors") })
            .forward(user_ws_tx)
            .map(|_tx_rx| ())
            .map_err(|ws_err| info!("WebSocket send error: {}", ws_err)),
    );

    // Save the sender in our list of connected ws_users.
    info!("ws_users.insert: {}", my_ws_uid);
    ws_users
        .lock()
        .expect("error uses.lock()")
        .insert(my_ws_uid, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.
    // Make an extra clone to give to our disconnection handler...
    // let users2 = ws_users.clone();
    // Clippy recommends this craziness instead of ws_users.clone()
    let users2 = Arc::<
        std::sync::Mutex<
            std::collections::HashMap<
                usize,
                futures::sync::mpsc::UnboundedSender<warp::ws::Message>,
            >,
        >,
    >::clone(&ws_users);

    user_ws_rx
        // Every time the user sends a message, call receive message
        .for_each(move |msg| {
            receive_message(my_ws_uid, &msg, &ws_users);
            Ok(())
        })
        // for_each will keep processing as long as the user stays
        // connected. Once they disconnect, then...
        .then(move |result| {
            user_disconnected(my_ws_uid, &users2);
            result
        })
        // If at any time, there was a WebSocket error, log here...
        .map_err(move |e| {
            info!("WebSocket error(uid={}): {}", my_ws_uid, e);
        })
}

/// on receive WebSocket message
fn receive_message(msg_sender_ws_uid: usize, message: &Message, ws_users: &WsUsers) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = message.to_str() {
        s
    } else {
        return;
    };

    let msg_string = msg.to_string();
    // info!("msg: {}", msg_string);

    // The ws server can receive 2 kinds of msgs:
    // 1. for the server to process
    // 2. to forward to receivers

    if let Ok(msg_to_server) = serde_json::from_str::<WsMessageToServer>(&msg_string) {
        match msg_to_server {
            WsMessageToServer::MsgRequestWsUid { msg_sender_ws_uid } => {
                info!("MsgRequestWsUid: {}", msg_sender_ws_uid);
                let j = serde_json::to_string(
                    &WsMessageFromServer::MsgResponseWsUid {
                        your_ws_uid: msg_sender_ws_uid,
                        server_version: env!("CARGO_PKG_VERSION").to_string(),
                         })
                    .expect("serde_json::to_string(&WsMessageData::MsgResponseWsUid { your_ws_uid: msg_sender_ws_uid })");
                info!("send MsgResponseWsUid: {}", j);
                match ws_users
                    .lock()
                    .expect("error ws_users.lock()")
                    .get(&msg_sender_ws_uid)
                    .unwrap()
                    .unbounded_send(Message::text(j))
                {
                    Ok(()) => (),
                    Err(_disconnected) => {}
                }
                // send to other ws_users for reconnect. Do nothing if there is not yet other ws_users.
                // send_to_msg_receivers(ws_users, msg_sender_ws_uid, &msg_string, &msg_receivers_json)
            }
            WsMessageToServer::MsgPing { msg_id } => {
                // info!("MsgPing: {}", msg_id);

                let j = unwrap!(serde_json::to_string(&WsMessageFromServer::MsgPong {
                    msg_id
                }));
                // info!("send MsgPong: {}", j);
                match ws_users
                    .lock()
                    .expect("error ws_users.lock()")
                    .get(&msg_sender_ws_uid)
                    .unwrap()
                    .unbounded_send(Message::text(j))
                {
                    Ok(()) => (),
                    Err(_disconnected) => {}
                }
            }
        }
    } else {
        // forward msg to receivers
        if let Ok(msg_for_receivers) = serde_json::from_str::<WsMessageForReceivers>(&msg_string) {
            send_to_msg_receivers(
                ws_users,
                msg_sender_ws_uid,
                &msg_string,
                &msg_for_receivers.msg_receivers_json,
            );
        }
    }
}

/// New message from this user send to all other players except sender.
fn send_to_msg_receivers(
    ws_users: &WsUsers,
    msg_sender_ws_uid: usize,
    msg_string: &str,
    msg_receivers_json: &str,
) {
    // info!("send_to_msg_receivers: {}", msg_string);

    let vec_msg_receivers: Vec<usize> = unwrap!(serde_json::from_str(msg_receivers_json));

    for (&uid, tx) in ws_users.lock().expect("error ws_users.lock()").iter() {
        let mut is_player;
        is_player = false;
        for &pl_ws_uid in &vec_msg_receivers {
            if pl_ws_uid == uid {
                is_player = true;
            }
        }
        if msg_sender_ws_uid != uid && is_player {
            match tx.unbounded_send(Message::text(String::from(msg_string))) {
                Ok(()) => (),
                Err(_disconnected) => {
                    // The tx is disconnected, our `user_disconnected` code
                    // should be happening in another task, nothing more to
                    // do here.
                }
            }
        }
    }
}

/// disconnect user
fn user_disconnected(my_ws_uid: usize, ws_users: &WsUsers) {
    info!("good bye user: {}", my_ws_uid);

    // Stream closed up, so remove from the user list
    ws_users.lock().expect("ws_users.lock").remove(&my_ws_uid);
}
// endregion
