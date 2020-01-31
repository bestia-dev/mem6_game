// gamedatamod.rs
//! structs and methods around game data

//region: use
use crate::divnicknamemod;

use mem6_common::{GameStatus, Player, WsMessage};

use serde_derive::{Serialize, Deserialize};
use unwrap::unwrap;
use rand::rngs::SmallRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::Rng;
use strum_macros::AsRefStr;
use web_sys::WebSocket;
//endregion

//region: struct, enum
///2d size (any UM -pixel, items, percent)
#[derive(Serialize, Deserialize, Clone)]
pub struct Size2d {
    ///horizontal
    pub hor: usize,
    ///vertical
    pub ver: usize,
}
///game metadata (for the vector)
#[derive(Serialize, Deserialize, Clone)]
pub struct GameMetadata {
    ///folder
    pub folder: String,
    ///name
    pub name: String,
    ///description
    pub description: String,
}

///games metadata vector
#[derive(Serialize, Deserialize, Clone)]
pub struct GamesMetadata {
    ///vec game_metadata
    pub vec_game_metadata: Vec<GameMetadata>,
}

///game config
#[derive(Serialize, Deserialize, Clone)]
pub struct GameConfig {
    ///card moniker - the text/name of the card
    ///the zero element is card face down or empty, example alphabet begins with index 01 : A
    pub card_moniker: Vec<String>,
    ///img filenames
    pub img_filename: Vec<String>,
    ///sound filenames
    pub sound_filename: Vec<String>,
    ///card image width
    pub card_width: usize,
    ///card image height
    pub card_height: usize,
    ///number of cards horizontally
    pub grid_items_hor: usize,
    ///number of card vertically
    pub grid_items_ver: usize,
}

///the 3 possible statuss of one card
#[derive(Serialize, Deserialize, AsRefStr)]
pub enum CardStatusCardFace {
    ///card face down
    Down,
    ///card face Up Temporary
    UpTemporary,
    ///card face up Permanently
    UpPermanently,
}
///all the data for one card
#[derive(Serialize, Deserialize)]
pub struct Card {
    ///card status
    pub status: CardStatusCardFace,
    ///field for src attribute for HTML element imagea and filename of card image
    pub card_number_and_img_src: usize,
    ///field for id attribute for HTML element image contains the card index
    pub card_index_and_id: usize,
}

///save the message in queue to resend it if timeout expires
#[derive(Serialize, Deserialize)]
pub struct MsgInQueue {
    ///the player that must ack the msg
    pub player_ws_uid: usize,
    ///the msg id is a random number
    pub msg_id: usize,
    ///the content of the message if it needs to be resend
    pub msg: WsMessage,
}

///game data
pub struct GameData {
    ///my ws client instance unique id. To not listen the echo to yourself.
    pub my_ws_uid: usize,
    ///my nickname
    pub my_nickname: String,
    ///What player am I
    pub my_player_number: usize,
    ///web socket. used it to send message onclick.
    pub ws: WebSocket,
    ///players data as vector of player struct
    pub players: Vec<Player>,
    ///the json string for the ws server to send msgs to other players only
    pub players_ws_uid: String,
    ///game status: StatusStartPage,Player1,Player2
    pub game_status: GameStatus,
    ///vector of cards
    pub card_grid_data: Vec<Card>,
    ///card index of first click
    pub card_index_of_first_click: usize,
    ///card index of second click
    pub card_index_of_second_click: usize,
    ///content folder name
    pub game_name: String,
    ///whose turn is now:  player 1,2,3,...
    pub player_turn: usize,
    ///content folders vector
    pub content_folders: Vec<String>,
    ///games meta data
    pub games_metadata: Option<GamesMetadata>,
    ///game_configs
    pub game_config: Option<GameConfig>,
    ///error text
    pub error_text: String,
    ///href
    pub href: String,
    ///href hash the local page #
    pub href_hash: String,
    /// is reconnect
    pub is_reconnect: bool,
    /// to not check it all the time
    pub is_fullscreen: bool,
    /// vector of msgs waiting for ack. If the 3 sec timeout passes it resends the same msg.
    pub msgs_waiting_ack: Vec<MsgInQueue>,
    /// show debug info on the smartphone screen
    pub show_debug_info: bool,
}
//endregion

impl GameData {
    ///prepare new random data
    pub fn prepare_random_data(&mut self) {
        let item_count_minus_one = unwrap!(unwrap!(self.game_config.as_ref())
            .card_moniker
            .len()
            .checked_sub(1));
        let players_count = self.players.len();
        let cards_count = unwrap!(players_count.checked_mul(unwrap!(unwrap!(self
            .game_config
            .as_ref())
        .grid_items_hor
        .checked_mul(unwrap!(self.game_config.as_ref()).grid_items_ver))));
        let random_count = unwrap!(cards_count.checked_div(2));
        //if the number of cards is bigger than the images, i choose all the images.
        //for the rest I use random.
        //integer division rounds toward zero
        let multiple: usize = unwrap!(random_count.checked_div(item_count_minus_one));
        let rest =
            unwrap!(random_count.checked_sub(unwrap!(item_count_minus_one.checked_mul(multiple))));

        /*
                logmod::debug_write(&format!(
                    "item_count_minus_one {}  players_count {} cards_count {} random_count {} multiple {} rest {}",
                    item_count_minus_one,players_count,cards_count,random_count,multiple,
                    rest,
                ));
        */
        //region: find random numbers between 1 and item_count
        //vec_of_random_numbers is 0 based
        let mut vec_of_random_numbers = Vec::new();
        let mut rng = SmallRng::from_entropy();
        vec_of_random_numbers.clear();
        for _i in 1..=rest {
            //how to avoid duplicates
            let mut num: usize;
            // a do-while is written as a  loop-break
            loop {
                //gen_range is lower inclusive, upper exclusive 26 + 1
                num = rng.gen_range(1, unwrap!(item_count_minus_one.checked_add(1)));
                if !vec_of_random_numbers.contains(&num) {
                    break;
                }
            }
            //push a pair of the same number
            vec_of_random_numbers.push(num);
            vec_of_random_numbers.push(num);
        }
        for _m in 1..=multiple {
            for i in 1..=item_count_minus_one {
                vec_of_random_numbers.push(i);
                vec_of_random_numbers.push(i);
            }
        }
        //endregion

        //region: shuffle the numbers
        let vrndslice = vec_of_random_numbers.as_mut_slice();
        vrndslice.shuffle(&mut rng);
        //endregion

        //region: create Cards from random numbers
        let mut card_grid_data = Vec::new();

        //Index 0 is special and reserved for FaceDown. Cards start with base 1
        let new_card = Card {
            status: CardStatusCardFace::Down,
            card_number_and_img_src: 0,
            card_index_and_id: 0,
        };
        card_grid_data.push(new_card);

        //create cards and push to the vector
        for (index, random_number) in vec_of_random_numbers.iter().enumerate() {
            let new_card = Card {
                status: CardStatusCardFace::Down,
                //dereference random number from iterator
                card_number_and_img_src: *random_number,
                //card base index will be 1. 0 is reserved for FaceDown.
                card_index_and_id: unwrap!(index.checked_add(1), "usize overflow"),
            };
            card_grid_data.push(new_card);
        }
        //endregion
        self.card_grid_data = card_grid_data;
        /*
        logmod::debug_write(&format!(
            "vec_of_random_numbers.len {} card_grid_data.len {}",
            vec_of_random_numbers.len(),
            self.card_grid_data.len()
        ));
        */
    }
    ///associated function: before join, there are not random numbers, just default cards.
    pub fn prepare_for_empty() -> Vec<Card> {
        //prepare 32 empty cards. The random is calculated only on MsgJoin.
        let mut card_grid_data = Vec::new();
        //I must prepare the 0 index, but then I don't use it ever.
        for i in 0..=32 {
            let new_card = Card {
                status: CardStatusCardFace::Down,
                card_number_and_img_src: 1,
                card_index_and_id: i,
            };
            card_grid_data.push(new_card);
        }
        card_grid_data
    }
    ///constructor of game data
    pub fn new(ws: WebSocket, my_ws_uid: usize) -> Self {
        let my_nickname = divnicknamemod::load_nickname();
        let mut players = Vec::new();
        players.push(Player {
            ws_uid: my_ws_uid,
            nickname: my_nickname.to_string(),
            points: 0,
        });
        let players_ws_uid = prepare_players_ws_uid(&players);

        //return from constructor
        GameData {
            card_grid_data: Self::prepare_for_empty(),
            card_index_of_first_click: 0,
            card_index_of_second_click: 0,
            ws,
            my_ws_uid,
            my_nickname,
            players,
            players_ws_uid,
            game_status: GameStatus::StatusStartPage,
            game_name: "alphabet".to_string(),
            my_player_number: 1,
            player_turn: 0,
            content_folders: vec![String::from("alphabet")],
            game_config: None,
            games_metadata: None,
            error_text: "".to_string(),
            href: "".to_string(),
            href_hash: "".to_string(),
            is_reconnect: false,
            is_fullscreen: false,
            msgs_waiting_ack: vec![],
            show_debug_info: false,
        }
    }
    /*
    ///check only if status StatusStartPage
    pub fn is_status_start_page(&self) -> bool {
        #[allow(clippy::wildcard_enum_match_arm)]
        match self.game_status {
            GameStatus::StatusStartPage => true,
            _ => false,
        }
    }
    */
}

/// from the vector of players prepare a json string for the ws server
/// so that it can send the msgs only to the players
pub fn prepare_players_ws_uid(players: &[Player]) -> String {
    let mut players_ws_uid = Vec::new();
    for pl in players {
        players_ws_uid.push(pl.ws_uid);
    }
    //return
    unwrap!(serde_json::to_string(&players_ws_uid))
}
