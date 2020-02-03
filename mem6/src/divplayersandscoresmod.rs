// divplayersandscoresmod.rs
//! renders the div that shows players and scores

#![allow(clippy::panic)]

//region: use

//endregion

///Render Component: player score
///Its private fields are a cache copy from `game_data` fields.
///They are used for rendering
///and for checking if the data has changed to invalidate the render cache.
#[derive(Default)]
pub struct PlayersAndScores {
    ///whose turn is now:  player 1 or 2
    player_turn: usize,
    ///my nickname
    my_nickname: String,
    ///my points
    my_points: usize,
    ///What player am I
    my_player_number: usize,
    ///my ws client instance unique id.
    my_ws_uid: usize,
}

impl PlayersAndScores {
    ///constructor
    pub fn new(my_ws_uid: usize) -> Self {
        PlayersAndScores {
            my_points: 0,
            my_player_number: 1,
            my_nickname: "Player".to_string(),
            player_turn: 0,
            my_ws_uid,
        }
    }
}
