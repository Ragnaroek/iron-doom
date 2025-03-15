use crate::fixed::{Fixed, new_fixed_u32};

const MAX_SEGS: usize = 32;

pub struct RenderState {
    pub view_x: Fixed,
    pub view_y: Fixed,

    pub view_angle: usize,

    pub clip_angle: usize,

    pub solid_segs: Vec<ClipRange>,
    pub tables: RenderTables,
}

impl RenderState {
    pub fn new() -> RenderState {
        RenderState {
            view_x: new_fixed_u32(0),
            view_y: new_fixed_u32(0),
            view_angle: 0,
            clip_angle: 0,
            tables: RenderTables::empty(),
            solid_segs: vec![ClipRange::zero(); MAX_SEGS],
        }
    }
}

#[derive(Copy, Clone)]
pub struct ClipRange {
    pub first: i32,
    pub last: i32,
}

impl ClipRange {
    pub fn zero() -> ClipRange {
        ClipRange { first: 0, last: 0 }
    }
}

pub struct RenderTables {
    pub view_angle_to_x: Vec<i32>,
}

impl RenderTables {
    pub fn empty() -> RenderTables {
        RenderTables {
            view_angle_to_x: Vec::with_capacity(0),
        }
    }
}
