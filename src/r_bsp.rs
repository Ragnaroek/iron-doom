#[cfg(test)]
#[path = "./r_bsp_test.rs"]
mod r_bsp_test;

use crate::{
    def::{Level, Node},
    doom_data::NF_SUBSECTOR,
    fixed::{FRAC_BITS, Fixed, ZERO, fixed_mul},
    r_state::RenderState,
};

pub fn render_bsp_node(render_state: &RenderState, level: &Level, bsp_num: usize) {
    println!("render node {}", bsp_num);
    if (bsp_num & NF_SUBSECTOR) != 0 {
        println!("NF={:x}", !NF_SUBSECTOR);
        subsector(level, bsp_num & !NF_SUBSECTOR);
        return;
    }

    let bsp = &level.nodes[bsp_num];

    let side = point_on_side(render_state.view_x, render_state.view_y, bsp);

    // Recursively divide front space.
    if side {
        render_bsp_node(render_state, level, bsp.children[1] as usize);
    } else {
        render_bsp_node(render_state, level, bsp.children[0] as usize);
    }

    // TODO divide backspace
}

// true = back side, false = front side
fn point_on_side(x: Fixed, y: Fixed, node: &Node) -> bool {
    if node.dx.is_zero() {
        if x <= node.x {
            return node.dy > ZERO;
        }
        return node.dy < ZERO;
    }

    if node.dy.is_zero() {
        if y <= node.y {
            return node.dx < ZERO;
        }
        return node.dx > ZERO;
    }

    let dx = x - node.x;
    let dy = y - node.y;

    // Try to quickly decide by looking at sign bits.
    if (node.dy ^ node.dx ^ dx ^ dy).to_i32() < 0 {
        if (node.dy ^ dx).to_i32() < 0 {
            return true;
        };
        return false;
    }

    let left = fixed_mul(node.dy >> FRAC_BITS, dx);
    let right = fixed_mul(dy, node.dx >> FRAC_BITS);

    if right < left {
        return false;
    }
    return true;
}

fn subsector(level: &Level, num: usize) {
    println!("subsector: {}", num);

    let sub = &level.subsectors[num];
    println!("sub = {}", sub.first_line);
}
