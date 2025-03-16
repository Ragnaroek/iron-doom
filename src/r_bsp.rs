#[cfg(test)]
#[path = "./r_bsp_test.rs"]
mod r_bsp_test;

use crate::{
    def::{Level, Node, Seg},
    doom_data::NF_SUBSECTOR,
    fixed::{FRAC_BITS, Fixed, ZERO, fixed_mul},
    r_state::RenderState,
    tables::{ANG_90, ANG_180, ANG_270, ANGLE_TO_FINE_SHIFT, TAN_TO_ANGLE, slope_div},
};

const BOX_TOP: usize = 0;
const BOX_BOTTOM: usize = 1;
const BOX_LEFT: usize = 2;
const BOX_RIGHT: usize = 3;

const CHECK_COORD: [[usize; 4]; 11] = [
    [3, 0, 2, 1],
    [3, 0, 2, 0],
    [3, 1, 2, 0],
    [0, 0, 0, 0],
    [2, 0, 2, 1],
    [0, 0, 0, 0],
    [3, 1, 3, 0],
    [0, 0, 0, 0],
    [2, 0, 3, 1],
    [2, 1, 3, 1],
    [2, 1, 3, 0],
];

pub fn render_bsp_node(render_state: &RenderState, level: &Level, bsp_num: usize) {
    println!("render node {}", bsp_num);
    if (bsp_num & NF_SUBSECTOR) != 0 {
        subsector(render_state, level, bsp_num & !NF_SUBSECTOR);
        return;
    }

    let bsp = &level.nodes[bsp_num];

    let side = point_on_side(render_state.view_x, render_state.view_y, bsp);

    // Recursively divide front space.
    render_bsp_node(render_state, level, bsp.children[side_ix(side)] as usize);

    // Possibly divide back space.
    if check_bbox(render_state, &bsp.bbox[side_ix(!side)]) {
        render_bsp_node(render_state, level, bsp.children[side_ix(!side)] as usize);
    }
}

fn side_ix(side: bool) -> usize {
    if side { 1 } else { 0 }
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

fn check_bbox(render_state: &RenderState, bsp_coord: &[Fixed; 4]) -> bool {
    let box_x;
    if render_state.view_x <= bsp_coord[BOX_LEFT] {
        box_x = 0;
    } else if render_state.view_x < bsp_coord[BOX_RIGHT] {
        box_x = 1;
    } else {
        box_x = 2;
    }

    let box_y;
    if render_state.view_y >= bsp_coord[BOX_TOP] {
        box_y = 0;
    } else if render_state.view_y > bsp_coord[BOX_BOTTOM] {
        box_y = 1;
    } else {
        box_y = 2;
    }

    let box_pos = (box_y << 2) + box_x;
    if box_pos == 5 {
        return true;
    }

    let x1 = bsp_coord[CHECK_COORD[box_pos][0]];
    let y1 = bsp_coord[CHECK_COORD[box_pos][1]];
    let x2 = bsp_coord[CHECK_COORD[box_pos][2]];
    let y2 = bsp_coord[CHECK_COORD[box_pos][3]];

    let mut angle1 = point_to_angle(render_state, x1, y1) - render_state.view_angle;
    let mut angle2 = point_to_angle(render_state, x2, y2) - render_state.view_angle;
    let span = angle1 - angle2;

    // Sitting on a line?
    if span >= ANG_180 {
        return true;
    }

    let mut t_span = angle1 + render_state.clip_angle;
    if t_span > 2 * render_state.clip_angle {
        t_span -= 2 * render_state.clip_angle;

        // Totally off the left edge?
        if t_span >= span {
            return false;
        }
        angle1 = render_state.clip_angle;
    }

    t_span = render_state.clip_angle - angle2;
    if t_span > 2 * render_state.clip_angle {
        t_span -= 2 * render_state.clip_angle;

        // Totally off the left edge?
        if t_span >= span {
            return false;
        }
        angle2 = render_state.clip_angle.wrapping_neg();
    }

    angle1 = (angle1 + ANG_90) >> ANGLE_TO_FINE_SHIFT;
    angle2 = (angle2 + ANG_90) >> ANGLE_TO_FINE_SHIFT;
    let sx1 = render_state.tables.view_angle_to_x[angle1];
    let mut sx2 = render_state.tables.view_angle_to_x[angle2];

    // Does not cross a pixel.
    if sx1 == sx2 {
        return false;
    }
    sx2 -= 1;

    let mut start = &render_state.solid_segs[0];
    for seg in &render_state.solid_segs {
        start = seg;

        if seg.last >= sx2 {
            break;
        }
    }

    if sx1 >= start.first && sx2 <= start.last {
        // The clippost contains the new span.
        return false;
    }

    return true;
}

fn point_to_angle(render_state: &RenderState, x_param: Fixed, y_param: Fixed) -> usize {
    let x = x_param - render_state.view_x;
    let y = y_param - render_state.view_y;

    if x.is_zero() && y.is_zero() {
        return 0;
    }

    if x >= ZERO {
        if y >= ZERO {
            if x > y {
                return TAN_TO_ANGLE[slope_div(y, x)];
            } else {
                return ANG_90 - 1 - TAN_TO_ANGLE[slope_div(x, y)];
            }
        } else {
            let y = -y;
            if x > y {
                return TAN_TO_ANGLE[slope_div(y, x)].wrapping_neg();
            } else {
                return ANG_270 + TAN_TO_ANGLE[slope_div(x, y)];
            }
        }
    } else {
        let x = -x;
        if y >= ZERO {
            if x > y {
                return ANG_180 - 1 - TAN_TO_ANGLE[slope_div(y, x)];
            } else {
                return ANG_90 + TAN_TO_ANGLE[slope_div(x, y)];
            }
        } else {
            let y = -y;
            if x > y {
                return ANG_180 + TAN_TO_ANGLE[slope_div(y, x)];
            } else {
                return ANG_270 - 1 - TAN_TO_ANGLE[slope_div(x, y)];
            }
        }
    }
}

// subsector
// Determine floor/ceiling planes.
// Add sprites of things in sector.
// Draw one or more line segments.
fn subsector(render_state: &RenderState, level: &Level, num: usize) {
    let sub = &level.subsectors[num];
    println!("sub = {}", sub.first_line);

    let front_sector = &level.sectors[sub.sector];
    let count = sub.num_lines;

    // TODO determine floor and ceilingplane
    // TODO AddSprites

    for i in 0..count {
        let line = &level.segs[(sub.first_line + i) as usize];
        add_line(line)
    }
}

fn add_line(seg: &Seg) {
    todo!("impl add_line");
}
