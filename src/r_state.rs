use crate::fixed::{Fixed, new_fixed_u32};

pub struct RenderState {
    pub view_x: Fixed,
    pub view_y: Fixed,
}

impl RenderState {
    pub fn new() -> RenderState {
        RenderState {
            view_x: new_fixed_u32(0),
            view_y: new_fixed_u32(0),
        }
    }
}
