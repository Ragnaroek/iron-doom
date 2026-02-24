use crate::{i_video::SCREEN_WIDTH, id_sdl::RenderContext, v_patch::Patch};

pub fn draw_patch(rdr: &mut RenderContext, x_param: usize, y_param: usize, patch: &Patch) {
    let y = y_param - patch.top_offset;
    let x = x_param - patch.left_offset;

    let mut dest_top = y * SCREEN_WIDTH + x;
    for column in &patch.columns {
        for post in &column.posts {
            let mut dest = dest_top + post.top_delta * SCREEN_WIDTH;
            for j in 0..post.length {
                rdr.set_buffer(dest, post.data[j]);
                dest += SCREEN_WIDTH;
            }
        }
        dest_top += 1;
    }
}
