use crate::{
    game::{GameState, State},
    id_sdl::RenderContext,
    v_patch::load_patch_lump,
    v_video::draw_patch,
    wad::check_num_for_name,
};

pub fn do_advance_demo(game_state: &mut GameState) {
    game_state.state = State::DemoScreen;
    game_state.page_name = "TITLEPIC".to_string();
}

pub fn page_drawer(rdr: &mut RenderContext, game_state: &mut GameState) {
    println!("### draw page: {}", game_state.page_name);
    let lump_ix =
        check_num_for_name(&game_state.lump_info, &game_state.page_name).expect("page lump");
    let patch = load_patch_lump(game_state, lump_ix).expect("page patch");
    // TODO: Cache the patch lump, so it does not have to be constructed all the time?
    draw_patch(rdr, 0, 0, &patch);
}
