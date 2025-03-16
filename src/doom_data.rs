pub const ML_VERTEXES: usize = 4;
pub const ML_SEGS: usize = 5;
pub const ML_SSECTORS: usize = 6;
pub const ML_NODES: usize = 7;
pub const ML_SECTORS: usize = 8;

// size in the WAD binary for MapNode
pub const MAP_VERTEX_SIZE: usize = 4;
pub const MAP_NODE_SIZE: usize = 28;
pub const MAP_SECTOR_SIZE: usize = 26;
pub const MAP_SUBSECTOR_SIZE: usize = 4;
pub const MAP_SEG_SIZE: usize = 12;

// Indicate a leaf.
pub const NF_SUBSECTOR: usize = 0x8000;

pub struct MapNode {}
