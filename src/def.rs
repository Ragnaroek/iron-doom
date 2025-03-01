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

pub struct Level {
    pub nodes: Vec<Node>,
}
