use std::os::unix::fs::FileExt;

use crate::{
    def::{Level, Node},
    doom_data::{MAP_NODE_SIZE, ML_NODES},
    fixed::{self, new_fixed_u32},
    game::GameState,
    util::DataReader,
    wad::check_num_for_name,
};

pub fn setup_level(game_state: &mut GameState) -> Result<Level, String> {
    let lump_name = format!("E{}M{}", game_state.episode, game_state.map);

    println!("name = {}", lump_name);

    let lump_num = check_num_for_name(&game_state.lump_info, &lump_name).expect("no lump for map");

    let nodes = load_nodes(game_state, lump_num + ML_NODES)?;

    let level = Level { nodes };

    Ok(level)
}

fn load_nodes(game_state: &mut GameState, lump: usize) -> Result<Vec<Node>, String> {
    let lump = &game_state.lump_info[lump];

    let num_nodes = lump.size / MAP_NODE_SIZE;

    let wad_file = &mut game_state.wad_files[lump.handle];
    let mut node_data = vec![0; lump.size];

    wad_file
        .read_exact_at(&mut node_data, lump.position as u64)
        .map_err(|e| e.to_string())?;

    let mut result = Vec::with_capacity(num_nodes);
    let mut reader = DataReader::new(&node_data);
    for _ in 0..num_nodes {
        let x = fixed::new_fixed_u16(reader.read_u16(), 0);
        let y = fixed::new_fixed_u16(reader.read_u16(), 0);
        let dx = fixed::new_fixed_u16(reader.read_u16(), 0);
        let dy = fixed::new_fixed_u16(reader.read_u16(), 0);

        let mut bbox = [[new_fixed_u32(0); 4]; 2];
        let mut children = [0; 2];
        for j in 0..2 {
            children[j] = reader.read_u16();
            for k in 0..4 {
                bbox[j][k] = fixed::new_fixed_u16(reader.read_u16(), 0)
            }
        }
        result.push(Node {
            x,
            y,
            dx,
            dy,
            bbox,
            children,
        });
    }
    Ok(result)
}
