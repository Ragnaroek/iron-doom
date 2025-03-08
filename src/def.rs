use crate::fixed::Fixed;

// BSP node.
pub struct Node {
    pub x: Fixed,
    pub y: Fixed,
    pub dx: Fixed,
    pub dy: Fixed,
    pub bbox: [[Fixed; 4]; 2],
    pub children: [u16; 2],
}

pub struct Sector {
    pub floor_height: Fixed,
    pub ceiling_height: Fixed,
    pub floor_pic: usize,
    pub ceiling_pic: usize,
    pub light_level: u16,
    pub special: u16,
    pub tag: u16,
}

pub struct Subsector {
    pub sector: usize,
    pub num_lines: u16,
    pub first_line: u16,
}

pub struct Level {
    pub sectors: Vec<Sector>,
    pub subsectors: Vec<Subsector>,
    pub nodes: Vec<Node>,
}
