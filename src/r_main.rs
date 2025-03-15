use crate::{game::GameState, r_bsp::render_bsp_node};

pub fn render_player_view(game_state: &GameState) {
    let level = game_state.current_level.as_ref().expect("level set");
    render_bsp_node(&game_state.render_state, level, level.nodes.len() - 1);
}
