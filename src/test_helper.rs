use std::path::PathBuf;

use crate::{def::Level, game::GameState, p_setup::setup_level, wad::init_multiple_files};

// Loads E1M1 from shareware data for testing.
pub fn setup_e1m1() -> Result<(GameState, Level), String> {
    let mut test_wad = PathBuf::new();
    test_wad.push("testdata/shareware_data/doom1.wad");
    let file_paths = vec![test_wad];
    let (files, lump_info) = init_multiple_files(&file_paths)?;

    let mut game_state = GameState::new(files, lump_info);
    game_state.episode = 1;
    game_state.map = 1;

    let level = setup_level(&mut game_state)?;

    Ok((game_state, level))
}
