use crate::{i_video::SCREEN_WIDTH, id_sdl::RenderContext, v_patch::Patch};

pub fn draw_patch(rdr: &mut RenderContext, x_param: usize, y_param: usize, patch: &Patch) {
    let y = y_param - patch.top_offset;
    let x = x_param - patch.left_offset;

    println!("### w={}, h={}", patch.width, patch.height);

    let mut dest_top = y * SCREEN_WIDTH + x;
    println!("### dest_top = {}", dest_top);
    for i in 0..patch.width {
        let col = &patch.columns[i];
        let mut dest = dest_top + col.top_delta * SCREEN_WIDTH;
        for j in 0..col.length {
            rdr.set_buffer(dest, col.data[j]);
            dest += SCREEN_WIDTH;
        }
        dest_top += 1;
    }

    println!("### patch drawed to buffer");
}
