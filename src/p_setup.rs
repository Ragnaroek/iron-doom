#[cfg(test)]
#[path = "./p_setup_test.rs"]
mod p_setup_test;

use std::os::unix::fs::FileExt;

use crate::{
    def::{Level, Node, Sector, Seg, Subsector, Vertex},
    doom_data::{
        MAP_NODE_SIZE, MAP_SECTOR_SIZE, MAP_SEG_SIZE, MAP_SUBSECTOR_SIZE, MAP_VERTEX_SIZE,
        ML_NODES, ML_SECTORS, ML_SEGS, ML_SSECTORS, ML_VERTEXES,
    },
    fixed::{self, FRAC_BITS, new_fixed_u32},
    game::GameState,
    util::DataReader,
    wad::{check_num_for_name, read_name},
};

pub fn setup_level(game_state: &mut GameState) -> Result<Level, String> {
    let lump_name = format!("E{}M{}", game_state.episode, game_state.map);

    println!("name = {}", lump_name);

    let lump_num = check_num_for_name(&game_state.lump_info, &lump_name).expect("no lump for map");

    let sectors = load_sectors(game_state, lump_num + ML_SECTORS)?;
    let subsectors = load_subsectors(game_state, lump_num + ML_SSECTORS)?;
    let nodes = load_nodes(game_state, lump_num + ML_NODES)?;
    let vertexes = load_vertexes(game_state, lump_num + ML_VERTEXES)?;
    let segs = load_segs(game_state, lump_num + ML_SEGS, &vertexes)?;

    let level = Level {
        sectors,
        subsectors,
        nodes,
        segs,
    };

    Ok(level)
}

fn load_subsectors(game_state: &mut GameState, lump_ix: usize) -> Result<Vec<Subsector>, String> {
    let subsector_data = load_lump(game_state, lump_ix)?;
    let lump = &game_state.lump_info[lump_ix];

    let num_subsectors = lump.size / MAP_SUBSECTOR_SIZE;
    let mut result = Vec::with_capacity(num_subsectors);
    let mut reader = DataReader::new(&subsector_data);
    for _ in 0..num_subsectors {
        let num_lines = reader.read_u16();
        let first_line = reader.read_u16();
        result.push(Subsector {
            sector: 0,
            num_lines,
            first_line,
        })
    }
    Ok(result)
}

fn load_nodes(game_state: &mut GameState, lump_ix: usize) -> Result<Vec<Node>, String> {
    let node_data = load_lump(game_state, lump_ix)?;
    let lump = &game_state.lump_info[lump_ix];

    let num_nodes = lump.size / MAP_NODE_SIZE;
    let mut result = Vec::with_capacity(num_nodes);
    let mut reader = DataReader::new(&node_data);
    for _ in 0..num_nodes {
        let x = fixed::new_fixed_u16(reader.read_u16(), 0);
        let y = fixed::new_fixed_u16(reader.read_u16(), 0);
        let dx = fixed::new_fixed_u16(reader.read_u16(), 0);
        let dy = fixed::new_fixed_u16(reader.read_u16(), 0);

        let mut bbox = [[new_fixed_u32(0); 4]; 2];
        for j in 0..2 {
            for k in 0..4 {
                bbox[j][k] = fixed::new_fixed_u16(reader.read_u16(), 0)
            }
        }

        let mut children = [0; 2];
        for j in 0..2 {
            children[j] = reader.read_u16();
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

fn load_sectors(game_state: &mut GameState, lump_ix: usize) -> Result<Vec<Sector>, String> {
    let sector_data = load_lump(game_state, lump_ix)?;
    let lump = &game_state.lump_info[lump_ix];

    let num_sectors = lump.size / MAP_SECTOR_SIZE;
    let mut result = Vec::with_capacity(num_sectors);
    let mut reader = DataReader::new(&sector_data);
    for _ in 0..num_sectors {
        let floor_height = fixed::new_fixed_u16(reader.read_u16(), 0);
        let ceiling_height = fixed::new_fixed_u16(reader.read_u16(), 0);

        let floor_lump_name = read_name(&mut reader, 8);
        let ceiling_lump_name = read_name(&mut reader, 8);
        let floor_pic =
            check_num_for_name(&game_state.lump_info, &floor_lump_name).expect("floor pic lump");
        let ceiling_pic = check_num_for_name(&game_state.lump_info, &ceiling_lump_name)
            .expect("ceiling pic lump");

        let light_level = reader.read_u16();
        let special = reader.read_u16();
        let tag = reader.read_u16();

        result.push(Sector {
            floor_height,
            ceiling_height,
            floor_pic,
            ceiling_pic,
            light_level,
            special,
            tag,
        });
    }
    Ok(result)
}

fn load_segs(
    game_state: &mut GameState,
    lump_ix: usize,
    vertexes: &Vec<Vertex>,
) -> Result<Vec<Seg>, String> {
    let seg_data = load_lump(game_state, lump_ix)?;
    let lump = &game_state.lump_info[lump_ix];

    let num_segs = lump.size / MAP_SEG_SIZE;
    let mut result = Vec::with_capacity(num_segs);
    let mut reader = DataReader::new(&seg_data);
    for _ in 0..num_segs {
        let v1_ix = reader.read_u16() as usize;
        let v2_ix = reader.read_u16() as usize;
        let angle = (reader.read_u16() as usize) << 16;
        let line_def = reader.read_u16() as usize;
        let side = reader.read_u16() as usize;
        let offset = new_fixed_u32((reader.read_u16() as u32) << 16);
        result.push(Seg {
            v1: vertexes[v1_ix].clone(),
            v2: vertexes[v2_ix].clone(),
            angle,
            offset,
            line_def,
            side,
        })
    }

    Ok(result)
}

fn load_vertexes(game_state: &mut GameState, lump_ix: usize) -> Result<Vec<Vertex>, String> {
    let vertex_data = load_lump(game_state, lump_ix)?;
    let lump = &game_state.lump_info[lump_ix];

    let num_vertex = lump.size / MAP_VERTEX_SIZE;
    let mut result = Vec::with_capacity(num_vertex);
    let mut reader = DataReader::new(&vertex_data);
    for _ in 0..num_vertex {
        let x = new_fixed_u32((reader.read_u16() as u32) << FRAC_BITS);
        let y = new_fixed_u32((reader.read_u16() as u32) << FRAC_BITS);
        result.push(Vertex { x, y })
    }
    Ok(result)
}

pub fn load_lump(game_state: &mut GameState, lump: usize) -> Result<Vec<u8>, String> {
    let lump = &game_state.lump_info[lump];

    let wad_file = &mut game_state.wad_files[lump.handle];
    let mut data = vec![0; lump.size];
    wad_file
        .read_exact_at(&mut data, lump.position as u64)
        .map_err(|e| e.to_string())?;

    Ok(data)
}
