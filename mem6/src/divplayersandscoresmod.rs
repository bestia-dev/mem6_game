// divplayersandscoresmod.rs
//! renders the div that shows players and scores

//region: use
use crate::gamedatamod::GameData;
use crate::utilsmod;
//use crate::logmod;

use unwrap::unwrap;
use dodrio::builder::{text};
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use typed_html::dodrio;
//endregion

///Render Component: player score
///Its private fields are a cache copy from `game_data` fields.
///They are used for rendering
///and for checking if the data has changed to invalidate the render cache.
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
    ///copies the data from game data to internal cache
    /// internal fiels are used to render component
    pub fn update_intern_cache(&mut self, game_data: &GameData) -> bool {
        /*
        logmod::debug_write(&format!(
            "update_intern_cache  my_player_number {}",
            &game_data.my_player_number
        ));
        */

        let mut is_invalidated;
        is_invalidated = false;
        if game_data.my_player_number > 0
            && !game_data.players.is_empty()
            && game_data.players.len() >= unwrap!(game_data.my_player_number.checked_sub(1))
            && self.my_points
                != unwrap!(
                    game_data
                        .players
                        .get(unwrap!(game_data.my_player_number.checked_sub(1))),
                    "game_data.players.get(game_data.my_player_number - 1)"
                )
                .points
        {
            self.my_points = unwrap!(
                game_data
                    .players
                    .get(unwrap!(game_data.my_player_number.checked_sub(1))),
                "game_data.players.get(game_data.my_player_number - 1)"
            )
            .points;
            is_invalidated = true;
        }
        if self.my_player_number != game_data.my_player_number {
            self.my_player_number = game_data.my_player_number;
            is_invalidated = true;
        }
        if self.player_turn != game_data.player_turn {
            self.player_turn = game_data.player_turn;
            is_invalidated = true;
        }
        if self.my_ws_uid != game_data.my_ws_uid {
            self.my_ws_uid = game_data.my_ws_uid;
            is_invalidated = true;
        }
        if self.my_nickname != game_data.my_nickname {
            self.my_nickname = game_data.my_nickname.clone();
            is_invalidated = true;
        }
        is_invalidated
    }
}

impl Render for PlayersAndScores {
    ///This rendering will be rendered and then cached . It will not be rerendered untill invalidation.
    ///It is invalidate, when the points change.
    ///html element with 1 score for this players
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        let text1 = bumpalo::format!(in bump, "{} {}: {} points",
        self.my_nickname,
        utilsmod::ordinal_numbers(self.my_player_number),
        self.my_points)
        .into_bump_str();
        //return
        dodrio!(bump,
        <div class="grid_container_players" style= "grid-template-columns: auto;">
            <div class= "grid_item" style="text-align: center;">
                {vec![
                    text(text1),
                ]}
            </div>
        </div>
        )
    }
}
