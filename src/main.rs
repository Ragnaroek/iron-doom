use std::{thread::sleep, time::Duration};

use id::{
    config::read_id_config,
    game::{GameAction, GameState},
    net::try_run_tics,
    wad::init_multiple_files,
};

extern crate id;

fn main() -> Result<(), String> {
    let id_config = read_id_config()?;

    //shareware wad for testing
    let mut wad_file = id_config.data.id_data.clone();
    wad_file.push("doom1.wad");

    let files = vec![wad_file];

    let lump_info = init_multiple_files(&files)?;

    let game_state = GameState::new(lump_info);

    doom_loop(game_state); // never returns
    Ok(())
}

fn doom_loop(mut game_state: GameState) {
    // TEST
    game_state.action = GameAction::LoadLevel;
    // END TEST

    loop {
        try_run_tics(&mut game_state);

        sleep(Duration::from_millis(28)); // dummy tic rate
    }
}
