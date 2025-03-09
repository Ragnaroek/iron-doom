use crate::{def::Level, doom_data::NF_SUBSECTOR};

pub fn render_bsp_node(level: &Level, bsp_num: usize) {
    println!("render node {}", bsp_num);
    if (bsp_num & NF_SUBSECTOR) != 0 {
        println!("NF={:x}", !NF_SUBSECTOR);
        subsector(level, bsp_num & !NF_SUBSECTOR);
        return;
    }

    let bsp = &level.nodes[bsp_num];

    let side = point_on_side();

    // Recursively divide front space.
    render_bsp_node(level, bsp.children[side] as usize);

    //TODO check back space
    println!("rendering node: {}", bsp_num);
}

fn point_on_side() -> usize {
    0 // TODO compute side
}

fn subsector(level: &Level, num: usize) {
    println!("subsector: {}", num);

    let sub = &level.subsectors[num];
    println!("sub = {}", sub.first_line);
}
