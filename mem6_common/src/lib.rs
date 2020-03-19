#![doc(
    html_favicon_url = "https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/icons-16.png"
)]
#![doc(
    html_logo_url = "https://github.com/LucianoBestia/mem6_game/raw/master/webfolder/mem6/images/icons-192.png"
)]
// region: lmake_readme insert "readme.md"
//! # mem6_common
//!
//! version: 2020.221.1322  
//!
//! **commons for mem6 wasm and server**  
//! Learning to code Rust for a http + WebSocket.  
//! Here are just the structures, that are in common between frontend and backend.  
//! Mostly because of the Messages.  

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
    clippy::missing_inline_in_public_items,
    // Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
// endregion

// region: use statements
use serde_derive::{Serialize, Deserialize};
// endregion

/// WsMessageToServer enum for WebSocket
/// The ws server will perform an action according to this type.
#[allow(clippy::pub_enum_variant_names)]
#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessageToServer {
    /// Request WebSocket Uid - first message to WebSocket server
    MsgRequestWsUid {
        /// ws client instance unique id. To not listen the echo to yourself.
        msg_sender_ws_uid: usize,
    },
    /// MsgPing
    MsgPing {
        /// random msg_id
        msg_id: u32,
    },
}

/// WsMessageFromServer enum for WebSocket
/// The ws server will send this kind of msgs.
#[allow(clippy::pub_enum_variant_names)]
#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessageFromServer {
    /// response from WebSocket server for first message
    MsgResponseWsUid {
        /// WebSocket Uid
        your_ws_uid: usize,
        /// server version
        server_version: String,
    },
    /// MsgPong
    MsgPong {
        /// random msg_id
        msg_id: u32,
    },
}

// The struct WsMessageForReceivers original is in the
// wasm project with all the fields.
// A copy of this struct is also in the server project, but without the msg_data field.
// They are the same struct, but the declaration is different, because
// the server does not need all the data.
// So we need to duplicate the declaration. Not ideal, but practical.

// endregion
