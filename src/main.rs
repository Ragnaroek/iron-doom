use std::{thread::sleep, time::Duration};

use id::{
    config::read_id_config,
    d_main::page_drawer,
    game::{GameAction, GameState, State},
    id_sdl::RenderContext,
    net::try_run_tics,
    p_setup::load_lump,
    r_main::render_player_view,
    wad::{check_num_for_name, init_multiple_files},
};

extern crate id;

fn main() -> Result<(), String> {
    let id_config = read_id_config()?;

    //shareware wad for testing
    let mut wad_file = id_config.data.id_data.clone();
    wad_file.push("doom1.wad");

    let file_paths = vec![wad_file];
    let (files, lump_info) = init_multiple_files(&file_paths)?;
    let mut game_state = GameState::new(files, lump_info);

    let pallets = load_pallets(&mut game_state)?;

    let rdr = RenderContext::init(&id_config, pallets)?;
    doom_loop(rdr, game_state); // never returns
    Ok(())
}

fn doom_loop(mut rdr: RenderContext, mut game_state: GameState) {
    // TEST
    game_state.action = GameAction::LoadLevel;
    // END TEST

    loop {
        println!("next frame");
        try_run_tics(&mut game_state);

        display(&mut rdr, &mut game_state);

        sleep(Duration::from_millis(28)); // dummy tic rate
    }
}

fn display(rdr: &mut RenderContext, game_state: &mut GameState) {
    if game_state.set_size_needed {
        execute_set_view_size(game_state);
    }

    match game_state.state {
        State::DemoScreen => page_drawer(rdr, game_state),
        _ => {}
    }

    if game_state.state == State::Level {
        render_player_view(game_state);
    }

    rdr.draw_frame();
}

fn execute_set_view_size(game_state: &mut GameState) {
    game_state.set_size_needed = false;

    init_texture_mapping(game_state);
}

fn init_texture_mapping(game_state: &mut GameState) {
    // TODO initialize viewangletox!
}

fn load_pallets(game_state: &mut GameState) -> Result<Vec<u8>, String> {
    let lump_num =
        check_num_for_name(&game_state.lump_info, "PLAYPAL").expect("no lump for PLAYPAL");
    load_lump(game_state, lump_num)
}
