use std::path::PathBuf;

use crate::{game::GameState, p_setup::setup_level, wad::init_multiple_files};

#[test]
fn test_setup_level() -> Result<(), String> {
    let mut test_wad = PathBuf::new();
    test_wad.push("testdata/shareware_data/doom1.wad");
    let file_paths = vec![test_wad];
    let (files, lump_info) = init_multiple_files(&file_paths)?;

    let mut game_state = GameState::new(files, lump_info);
    game_state.episode = 1;
    game_state.map = 1;

    let level = setup_level(&mut game_state)?;

    assert_eq!(level.nodes.len(), 236);
    assert_eq!(level.nodes[235].children[0], 128);
    assert_eq!(level.nodes[235].children[1], 234);

    Ok(())
}
