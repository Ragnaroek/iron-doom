use crate::game::{GameState, game_ticker};

pub fn try_run_tics(game_state: &mut GameState) {
    game_ticker(game_state);
}
