use crate::{
    doom_data::ML_NODES,
    game::{GameAction, GameState},
    wad::check_num_for_name,
};

pub fn setup_level(game_state: &mut GameState) {
    let lump_name = format!("E{}M{}", game_state.episode, game_state.map);

    println!("name = {}", lump_name);

    let lump_num = check_num_for_name(&game_state.lump_info, &lump_name).expect("no lump for map");

    load_nodes(lump_num + ML_NODES);
}

fn load_nodes(lump: usize) {
    println!("loading nodes at {}", lump);
    // TODO load lump data from WAD
    // Parse node data to Node struct
}
