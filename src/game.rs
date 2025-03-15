use std::fs::File;

use crate::{def::Level, p_setup::setup_level, r_state::RenderState, wad::LumpInfo};

pub struct GameState {
    pub wad_files: Vec<File>,
    pub lump_info: Vec<LumpInfo>,
    pub action: GameAction,

    pub episode: usize,
    pub map: usize,
    pub current_level: Option<Level>,
    pub render_state: RenderState,

    pub set_size_needed: bool,
}

impl GameState {
    pub fn new(wad_files: Vec<File>, lump_info: Vec<LumpInfo>) -> GameState {
        GameState {
            wad_files,
            lump_info,
            action: GameAction::Nothing,
            episode: 1,
            map: 1,
            current_level: None,
            render_state: RenderState::new(),
            set_size_needed: false,
        }
    }
}

pub enum GameAction {
    Nothing,
    LoadLevel,
}

pub fn game_ticker(game_state: &mut GameState) {
    match game_state.action {
        GameAction::LoadLevel => do_load_level(game_state),
        GameAction::Nothing => { /* do nothing */ }
    }
}

// loads the selected level data and updates the 'current_level'
// in the game_state.
fn do_load_level(game_state: &mut GameState) {
    let level = setup_level(game_state).expect("level setup");
    game_state.current_level = Some(level);
    game_state.action = GameAction::Nothing;
}
