use crate::{setup::setup_level, wad::LumpInfo};

pub struct GameState {
    pub lump_info: Vec<LumpInfo>,
    pub action: GameAction,

    pub episode: usize,
    pub map: usize,
    pub level_state: Option<LevelState>,
}

impl GameState {
    pub fn new(lump_info: Vec<LumpInfo>) -> GameState {
        GameState {
            lump_info,
            action: GameAction::Nothing,
            episode: 1,
            map: 1,
            level_state: None,
        }
    }
}

pub struct LevelState {}

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

fn do_load_level(game_state: &mut GameState) {
    setup_level(game_state);

    game_state.action = GameAction::Nothing;
}
