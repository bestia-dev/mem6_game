// game_data_mod.rs
//! structs and methods around game data

// region: use
use crate::*;

use serde_derive::{Serialize, Deserialize};
use unwrap::unwrap;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng, Rng};
use strum_macros::{Display, AsRefStr, EnumString};
//use strum::{EnumString};
// endregion: use

// region: struct, enum

/// videos
#[derive(Serialize, Deserialize, Clone)]
pub struct Videos {
    /// vec of strings
    pub videos: Vec<String>,
}

/// audio
#[derive(Serialize, Deserialize, Clone)]
pub struct Audio {
    /// vec of strings
    pub audio: Vec<String>,
}

/// 2d size (any UM -pixel, items, percent)
#[derive(Serialize, Deserialize, Clone)]
pub struct Size2d {
    /// horizontal
    pub hor: usize,
    /// vertical
    pub ver: usize,
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

/// the game can be in various statuses and that differentiate the UI and actions
/// all players have the same game status
/// when sending a message, I will send a String with EnumString
#[derive(Display, AsRefStr, EnumString, Serialize, Deserialize, Clone, PartialEq)]
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

/// game metadata (for the vector)
#[derive(Serialize, Deserialize, Clone)]
pub struct GameMetadata {
    /// folder
    pub folder: String,
    /// name
    pub name: String,
    /// description
    pub description: String,
}

/// games metadata vector
#[derive(Serialize, Deserialize, Clone)]
pub struct GamesMetadata {
    /// vec game_metadata
    pub vec_game_metadata: Vec<GameMetadata>,
}

/// game config
#[derive(Serialize, Deserialize, Clone)]
pub struct GameConfig {
    /// card moniker - the text/name of the card
    /// the zero element is card face down or empty, example alphabet begins with index 01 : A
    pub card_moniker: Vec<String>,
    /// img filenames
    pub img_filename: Vec<String>,
    /// sound filenames
    pub sound_filename: Vec<String>,
    /// card image width
    pub card_width: usize,
    /// card image height
    pub card_height: usize,
    /// number of cards horizontally
    pub grid_items_hor: usize,
    /// number of card vertically
    pub grid_items_ver: usize,
    /// is there big image available
    pub big_img: bool,
}

/// the 3 possible statuses of one card
#[derive(Serialize, Deserialize, AsRefStr)]
pub enum CardStatusCardFace {
    /// card face down
    Down,
    /// card face Up Temporary
    UpTemporary,
    /// card face up Permanently
    UpPermanently,
}
/// all the data for one card
#[derive(Serialize, Deserialize)]
pub struct Card {
    /// card status
    pub status: CardStatusCardFace,
    /// field for src attribute for HTML element image and filename of card image
    pub card_number: usize,
    /// field for id attribute for HTML element image contains the card index
    pub card_index_and_id: usize,
}

/// game data
pub struct GameData {
    /// my nickname
    pub my_nickname: String,
    /// What player am I
    pub my_player_number: usize,
    /// group_id is the ws_uid of the first player
    pub group_id: usize,
    /// players data as vector of player struct
    pub players: Vec<Player>,
    /// game status: StatusStartPage,Player1,Player2
    pub game_status: GameStatus,
    /// vector of cards
    pub card_grid_data: Vec<Card>,
    /// card index of first click
    pub card_index_of_1st_click: usize,
    /// card index of second click
    pub card_index_of_2nd_click: usize,
    /// content folder name
    pub game_name: String,
    /// whose turn is now:  player 1,2,3,...
    pub player_turn: usize,
    /// content folders vector
    pub content_folders: Vec<String>,
    /// games meta data
    pub games_metadata: Option<GamesMetadata>,
    /// game_configs
    pub game_config: Option<GameConfig>,
    /// videos links for fun
    pub videos: Vec<String>,
    /// audio for fun
    pub audio: Vec<String>,
    /// sounds and labels toggle
    pub sounds_and_labels: bool,
}

/// `WsMessageGameData` enum for WebSocket
#[allow(clippy::pub_enum_variant_names)]
#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessageGameData {
    /// join the group
    MsgJoin {
        /// my nickname
        my_nickname: String,
    },
    /// player1 initialize the game data and sends it to all players
    /// I will send json string to not confuse the server with vectors
    MsgStartGame {
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
        /// have to send all the state of the game
        card_index_of_1st_click: usize,
        /// msg id (random)
        msg_id: usize,
    },
    /// player click success
    MsgClick2ndCard {
        /// have to send all the state of the game
        card_index_of_2nd_click: usize,
        /// is point
        is_point: bool,
        /// msg id (random)
        msg_id: usize,
    },
    /// drink end
    MsgDrinkEnd {},
    /// Game Over
    MsgGameOver {},
    /// Play Again
    MsgPlayAgain {},
    /// player change
    MsgTakeTurn {
        /// msg id (random)
        msg_id: usize,
    },
    /// sounds and labels toggle
    MsgSoundsAndLabels {
        /// sounds and labels toggle
        sounds_and_labels: bool,
    },
    /// acknowledge msg, that the receiver received the message
    MsgAck {
        /// msg id (random)
        msg_id: usize,
        /// kind of ack msg
        msg_ack_kind: MsgAckKind,
    },
    /// ask player1 for resync
    MsgAskPlayer1ForResync {},
    /// all game data
    MsgAllGameData {
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
        /// game status (isize is the enum variant datatype)
        game_status: String,
    },
    /// webrtc offer msg
    MsgWebrtcOffer { sdp: String },
    /// webrtc answer msg
    MsgWebrtcAnswer { sdp: String },
    /// webrtc answer msg
    MsgWebrtcIceCandidate { sdp: String },
    // chat msg to send over webrtc
    //MsgWebrtcChatMsg { text: String },
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

impl GameData {
    /// constructor of game data
    pub fn new(my_ws_uid: usize) -> Self {
        let my_nickname = storage_mod::load_nickname();
        let mut players = Vec::new();
        players.push(Player {
            ws_uid: my_ws_uid,
            nickname: my_nickname.to_string(),
            points: 0,
        });

        // return from constructor
        GameData {
            card_grid_data: Self::prepare_for_empty(),
            card_index_of_1st_click: 0,
            card_index_of_2nd_click: 0,
            my_nickname,
            group_id: 0,
            players,
            game_status: GameStatus::StatusStartPage,
            game_name: "alphabet".to_string(),
            my_player_number: 1,
            player_turn: 0,
            content_folders: vec![String::from("alphabet")],
            game_config: None,
            games_metadata: None,
            videos: vec![],
            audio: vec![],
            sounds_and_labels: true,
        }
    }
    /// reset the data to play again the game
    pub fn reset_for_play_again(&mut self) {
        self.card_index_of_1st_click = 0;
        self.card_index_of_2nd_click = 0;
        // reset points
        for x in &mut self.players {
            x.points = 0;
        }
    }
    /// prepare new random data
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
        // if the number of cards is bigger than the images, i choose all the images.
        // for the rest I use random.
        // integer division rounds toward zero
        let multiple: usize = unwrap!(random_count.checked_div(item_count_minus_one));
        let rest =
            unwrap!(random_count.checked_sub(unwrap!(item_count_minus_one.checked_mul(multiple))));

        /*
                // websysmod::debug_write(&format!(
                    "item_count_minus_one {}  players_count {} cards_count {} random_count {} multiple {} rest {}",
                    item_count_minus_one,players_count,cards_count,random_count,multiple,
                    rest,
                ));
        */
        // region: find random numbers between 1 and item_count
        // vec_of_random_numbers is 0 based
        let mut vec_of_random_numbers = Vec::new();
        let mut rng = SmallRng::from_entropy();
        vec_of_random_numbers.clear();
        for _i in 1..=rest {
            // how to avoid duplicates
            let mut num: usize;
            // a do-while is written as a  loop-break
            loop {
                // gen_range is lower inclusive, upper exclusive 26 + 1
                num = rng.gen_range(1, unwrap!(item_count_minus_one.checked_add(1)));
                if !vec_of_random_numbers.contains(&num) {
                    break;
                }
            }
            // push a pair of the same number
            vec_of_random_numbers.push(num);
            vec_of_random_numbers.push(num);
        }
        for _m in 1..=multiple {
            for i in 1..=item_count_minus_one {
                vec_of_random_numbers.push(i);
                vec_of_random_numbers.push(i);
            }
        }
        // endregion

        // region: shuffle the numbers
        let rnd_slice = vec_of_random_numbers.as_mut_slice();
        rnd_slice.shuffle(&mut rng);
        // endregion

        // region: create Cards from random numbers
        let mut card_grid_data = Vec::new();

        // Index 0 is special and reserved for FaceDown. Cards start with base 1
        let new_card = Card {
            status: CardStatusCardFace::Down,
            card_number: 0,
            card_index_and_id: 0,
        };
        card_grid_data.push(new_card);

        // create cards and push to the vector
        for (index, random_number) in vec_of_random_numbers.iter().enumerate() {
            let new_card = Card {
                status: CardStatusCardFace::Down,
                // dereference random number from iterator
                card_number: *random_number,
                // card base index will be 1. 0 is reserved for FaceDown.
                card_index_and_id: unwrap!(index.checked_add(1), "usize overflow"),
            };
            card_grid_data.push(new_card);
        }
        // endregion
        self.card_grid_data = card_grid_data;
        /*
        // websysmod::debug_write(&format!(
            "vec_of_random_numbers.len {} card_grid_data.len {}",
            vec_of_random_numbers.len(),
            self.card_grid_data.len()
        ));
        */
    }
    /// associated function: before join, there are not random numbers, just default cards.
    pub fn prepare_for_empty() -> Vec<Card> {
        // prepare 32 empty cards. The random is calculated only on MsgJoin.
        let mut card_grid_data = Vec::new();
        // I must prepare the 0 index, but then I don't use it ever.
        for i in 0..=32 {
            let new_card = Card {
                status: CardStatusCardFace::Down,
                card_number: 1,
                card_index_and_id: i,
            };
            card_grid_data.push(new_card);
        }
        card_grid_data
    }
    /// from the vector of players prepare a json string for the ws server
    /// so that it can send the msgs only to the players
    pub fn prepare_json_msg_receivers(&self) -> String {
        let mut vec_msg_receivers = Vec::new();
        for pl in &self.players {
            vec_msg_receivers.push(pl.ws_uid);
        }
        // return
        unwrap!(serde_json::to_string(&vec_msg_receivers))
    }
    /// every smartphone grid starts and ends at a specific index of the card vector
    pub fn grid_start_end_index(&self) -> (usize, usize) {
        let start_index = unwrap!(unwrap!((unwrap!(self.my_player_number.checked_sub(1)))
            .checked_mul(unwrap!(unwrap!(self.game_config.as_ref())
                .grid_items_hor
                .checked_mul(unwrap!(self.game_config.as_ref()).grid_items_ver))))
        .checked_add(1));

        let mut end_index = unwrap!(self.my_player_number.checked_mul(unwrap!(unwrap!(self
            .game_config
            .as_ref())
        .grid_items_hor
        .checked_mul(unwrap!(self.game_config.as_ref()).grid_items_ver))));
        // the count of cards can now be not divisible with 2 for card pairs.
        // so I need to make a different last card that is not clickable.
        if end_index >= self.card_grid_data.len() {
            #[allow(clippy::integer_arithmetic)]
            {
                end_index -= 1;
            }
        }
        // return
        (start_index, end_index)
    }

    /// which player is me
    #[allow(clippy::integer_arithmetic)]
    pub fn my_player(&self) -> &Player {
        // players vector are counted from zero
        // player_turn is counted from 1
        unwrap!(self.players.get(self.my_player_number - 1))
    }

    /// which player is me
    #[allow(clippy::integer_arithmetic)]
    pub fn my_player_mut(&mut self) -> &mut Player {
        // players vector are counted from zero
        // player_turn is counted from 1
        unwrap!(self.players.get_mut(self.my_player_number - 1))
    }
    /// which player is on turn now
    #[allow(clippy::integer_arithmetic)]
    pub fn player_turn_now(&self) -> &Player {
        // players vector are counted from zero
        // player_turn is counted from 1
        unwrap!(self.players.get(self.player_turn - 1))
    }

    /// which player is on turn now
    #[allow(clippy::integer_arithmetic)]
    pub fn player_turn_now_mut(&mut self) -> &mut Player {
        // players vector are counted from zero
        // player_turn is counted from 1
        unwrap!(self.players.get_mut(self.player_turn - 1))
    }

    /// is my turn?
    pub const fn is_my_turn(&self) -> bool {
        self.my_player_number == self.player_turn
    }

    /// get 1st card
    pub fn get_1st_card(&self) -> &Card {
        unwrap!(self.card_grid_data.get(self.card_index_of_1st_click))
    }

    /// get 1st card mut
    pub fn get_1st_card_mut(&mut self) -> &mut Card {
        unwrap!(self.card_grid_data.get_mut(self.card_index_of_1st_click))
    }

    /// get 2nd card
    pub fn get_2nd_card(&self) -> &Card {
        unwrap!(self.card_grid_data.get(self.card_index_of_2nd_click))
    }

    /// get 2nd card mut
    pub fn get_2nd_card_mut(&mut self) -> &mut Card {
        unwrap!(self.card_grid_data.get_mut(self.card_index_of_2nd_click))
    }
}


