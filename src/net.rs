use crate::{
    d_main::do_advance_demo,
    game::{GameState, game_ticker},
};

pub fn try_run_tics(game_state: &mut GameState) {
    if game_state.advance_demo {
        do_advance_demo(game_state);
    }
    game_ticker(game_state); // G_Ticker
}
