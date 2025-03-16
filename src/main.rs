use std::{
    f64::consts::PI,
    process::exit,
    thread::sleep,
    time::{Duration, Instant},
};

use id::{
    config::read_id_config,
    game::{GameAction, GameState},
    net::try_run_tics,
    r_main::render_player_view,
    wad::init_multiple_files,
};
use sdl3::{
    EventPump,
    event::Event,
    pixels::{Color, PixelFormat},
    render::{Canvas, Texture},
    sys::pixels::SDL_PixelFormat,
    video::Window,
};

extern crate id;

fn main() -> Result<(), String> {
    let id_config = read_id_config()?;

    //shareware wad for testing
    let mut wad_file = id_config.data.id_data.clone();
    wad_file.push("doom1.wad");

    let file_paths = vec![wad_file];
    let (files, lump_info) = init_multiple_files(&file_paths)?;
    let game_state = GameState::new(files, lump_info);

    let rdr = RenderContext::init()?;
    doom_loop(rdr, game_state); // never returns
    Ok(())
}

fn doom_loop(mut rdr: RenderContext, mut game_state: GameState) {
    // TEST
    game_state.action = GameAction::LoadLevel;
    // END TEST

    loop {
        println!("next frame");
        try_run_tics(&mut game_state);

        display(&mut rdr, &mut game_state);

        sleep(Duration::from_millis(28)); // dummy tic rate
    }
}

fn display(rdr: &mut RenderContext, game_state: &mut GameState) {
    if game_state.set_size_needed {
        execute_set_view_size(game_state);
    }

    render_player_view(game_state);

    rdr.draw_frame();
}

fn execute_set_view_size(game_state: &mut GameState) {
    game_state.set_size_needed = false;

    init_texture_mapping(game_state);
}

fn init_texture_mapping(game_state: &mut GameState) {
    // TODO initialize viewangletox!
}

// As for now only for SDL, idea is to support different
// render backends through this
pub struct RenderContext {
    canvas: Canvas<Window>,
    texture: Texture,
    event_pump: EventPump,

    start_time: Instant,
}

impl RenderContext {
    pub fn init() -> Result<RenderContext, String> {
        let sdl = sdl3::init().map_err(|e| e.to_string())?;
        let vid = sdl.video().map_err(|e| e.to_string())?;
        let event_pump = sdl.event_pump().map_err(|e| e.to_string())?;

        let w = 640;
        let h = 400;

        let mut win_builder = vid.window("DOOM", w, h);
        win_builder.position_centered();
        let win = win_builder.build().map_err(|e| e.to_string())?;

        let mut canvas = win.into_canvas();
        canvas.clear();
        canvas.present();

        let texture_builder = canvas.texture_creator();
        let texture = texture_builder
            .create_texture_target(
                unsafe { PixelFormat::from_ll(SDL_PixelFormat::XRGB8888) },
                w,
                h,
            )
            .map_err(|e| e.to_string())?;

        let start_time = Instant::now();
        Ok(RenderContext {
            canvas,
            texture,
            event_pump,

            start_time,
        })
    }

    pub fn draw_frame(&mut self) {
        // TODO update sdl texture from framebuffer!

        // dummy render
        let elapsed_time = self.start_time.elapsed().as_secs_f64();
        let red = 0.5 + 0.5 * (elapsed_time * 2.0 * PI).sin();
        let green = 0.5 + 0.5 * ((elapsed_time * 2.0 * PI) + (2.0 * PI / 3.0)).sin();
        let blue = 0.5 + 0.5 * ((elapsed_time * 2.0 * PI) + (4.0 * PI / 3.0)).sin();
        self.canvas.set_draw_color(Color::RGB(
            (red * 255.0) as u8,
            (green * 255.0) as u8,
            (blue * 255.0) as u8,
        ));
        self.canvas.clear();
        self.canvas.present();

        self.handle_keys();
    }

    fn handle_keys(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => exit(0),
                _ => {}
            }
        }
    }
}
