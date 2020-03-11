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
use strum_macros::{Display, AsRefStr};
use serde_derive::{Serialize, Deserialize};
// endregion

/// `WsMessage` enum for WebSocket
#[allow(clippy::pub_enum_variant_names)]
#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessage {
    /// MsgDummy
    MsgDummy {
        /// anything
        dummy: String,
    },
    /// Request WebSocket Uid - first message to WebSocket server
    MsgRequestWsUid {
        /// ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        /// json of vector of players for the server to know whom to send msg
        json_msg_receivers: String,
    },
    /// response from WebSocket server for first message
    MsgResponseWsUid {
        /// WebSocket Uid
        your_ws_uid: usize,
        /// server version
        server_version: String,
    },
    /// MsgPing
    MsgPing {
        /// random msg_id
        msg_id: u32,
    },
    /// MsgPong
    MsgPong {
        /// random msg_id
        msg_id: u32,
    },
    /// join the group
    MsgJoin {
        /// ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        /// json of vector of players for the server to know whom to send msg
        json_msg_receivers: String,
        /// my nickname
        my_nickname: String,
    },
    /// player1 initialize the game data and sends it to all players
    /// I will send json string to not confuse the server with vectors
    MsgStartGame {
        /// ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        /// json of vector of players for the server to know whom to send msg
        json_msg_receivers: String,
        /// json of vector of players with nicknames and order data
        players: String,
        /// vector of cards status
        card_grid_data: String,
        /// json of game_config
        game_config: String,
        /// game name
        game_name: String,
        /// player turn to start game
        player_turn: usize,
    },
    /// player click
    MsgClick1stCard {
        /// this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        /// all players for the server to know whom to send msg
        json_msg_receivers: String,
        /// have to send all the state of the game
        card_index_of_1st_click: usize,
        /// msg id (random)
        msg_id: usize,
    },
    /// player click success
    MsgClick2ndCard {
        /// this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        /// all players for the server to know whom to send msg
        json_msg_receivers: String,
        /// have to send all the state of the game
        card_index_of_2nd_click: usize,
        /// is point
        is_point: bool,
        /// msg id (random)
        msg_id: usize,
    },
    /// drink end
    MsgDrinkEnd {
        /// this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        /// all players for the server to know whom to send msg
        json_msg_receivers: String,
    },
    /// Game Over
    MsgGameOver {
        /// this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        /// all players for the server to know whom to send msg
        json_msg_receivers: String,
    },
    /// Play Again
    MsgPlayAgain {
        /// this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        /// all players for the server to know whom to send msg
        json_msg_receivers: String,
    },
    /// player change
    MsgTakeTurn {
        /// ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        /// all players for the server to know whom to send msg
        json_msg_receivers: String,
        /// msg id (random)
        msg_id: usize,
    },
    /// acknowledge msg, that the receiver received the message
    MsgAck {
        /// msg sender uid
        my_ws_uid: usize,
        /// send msg to this players
        json_msg_receivers: String,
        /// msg id (random)
        msg_id: usize,
        /// kind of ack msg
        msg_ack_kind: MsgAckKind,
    },
    /// ask player1 for resync
    MsgAskPlayer1ForResync {
        /// msg sender uid
        my_ws_uid: usize,
        /// send msg to this players
        json_msg_receivers: String,
    },
    /// all game data
    MsgAllGameData {
        /// ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        /// only the players that reconnected
        json_msg_receivers: String,
        /// json of vector of players with nicknames and order data
        players: String,
        /// vector of cards status
        card_grid_data: String,
        /// have to send all the state of the game
        card_index_of_1st_click: usize,
        /// have to send all the state of the game
        card_index_of_2nd_click: usize,
        /// whose turn is now:  player 1,2,3,...
        player_turn: usize,
        /// game status
        game_status: GameStatus,
    },
}

/// the game can be in various statuses and that differentiate the UI and actions
/// all players have the same game status
#[derive(Display, AsRefStr, Serialize, Deserialize, Clone, PartialEq)]
#[allow(clippy::pub_enum_variant_names)]
pub enum GameStatus {
    /// start page
    StatusStartPage,
    /// Joined
    StatusJoined,
    /// before first card
    Status1stCard,
    /// before second card
    Status2ndCard,
    /// drink
    StatusDrink,
    /// take turn end
    StatusTakeTurn,
    /// game over
    StatusGameOver,
    /// StatusReconnect after a lost connection
    StatusReconnect,
    /// waiting ack msg
    StatusWaitingAckMsg,
}

/// data for one player
#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    /// ws_uid
    pub ws_uid: usize,
    /// nickname
    pub nickname: String,
    /// field for src attribute for HTML element image and filename of card image
    pub points: usize,
}

#[derive(Display, AsRefStr, Serialize, Deserialize, Clone)]
#[allow(clippy::pub_enum_variant_names)]
/// msg ack kind
pub enum MsgAckKind {
    /// ack for MsgTakeTurn
    MsgTakeTurn,
    /// ack for MsgClick1stCard
    MsgClick1stCard,
    /// ack for MsgClick2ndCard
    MsgClick2ndCard,
}

// endregion
