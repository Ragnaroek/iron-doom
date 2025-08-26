use std::process::exit;

use sdl3::{
    EventPump,
    event::Event,
    pixels::PixelFormat,
    render::{Canvas, Texture},
    sys::pixels::SDL_PixelFormat,
    video::Window,
};

use crate::{
    config::IDConfig,
    i_video::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

// As for now only for SDL, idea is to support different
// render backends through this
pub struct RenderContext {
    canvas: Canvas<Window>,
    texture: Texture,
    event_pump: EventPump,

    pallets: Vec<u8>,
    mul_width: usize,
    mul_height: usize,
    video_buffer: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl RenderContext {
    pub fn init(id_config: &IDConfig, pallets: Vec<u8>) -> Result<RenderContext, String> {
        if id_config.options.height % SCREEN_HEIGHT != 0 {
            return Err("screen height must be a multiple of 200".to_string());
        }
        let mul_height = id_config.options.height / SCREEN_HEIGHT;

        if id_config.options.width % SCREEN_WIDTH != 0 {
            return Err("screen width must be a multiple of 320".to_string());
        }
        let mul_width = id_config.options.width / SCREEN_WIDTH;

        println!("### mul w = {}", mul_width);
        println!("### mul h = {}", mul_height);

        let sdl = sdl3::init().map_err(|e| e.to_string())?;
        let vid = sdl.video().map_err(|e| e.to_string())?;
        let event_pump = sdl.event_pump().map_err(|e| e.to_string())?;

        let mut win_builder = vid.window(
            "DOOM",
            id_config.options.width as u32,
            id_config.options.height as u32,
        );
        win_builder.position_centered();
        let win = win_builder.build().map_err(|e| e.to_string())?;

        let mut canvas = win.into_canvas();
        canvas.clear();
        canvas.present();

        let texture_builder = canvas.texture_creator();
        let texture = texture_builder
            .create_texture_streaming(
                unsafe { PixelFormat::from_ll(SDL_PixelFormat::RGB24) },
                id_config.options.width as u32,
                id_config.options.height as u32,
            )
            .map_err(|e| e.to_string())?;

        Ok(RenderContext {
            canvas,
            texture,
            event_pump,

            pallets,
            mul_width,
            mul_height,
            video_buffer: [0; SCREEN_WIDTH * SCREEN_HEIGHT],
        })
    }

    pub fn draw_frame(&mut self) {
        self.texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                let mut src_offset = 0;
                for y in 0..SCREEN_HEIGHT {
                    for x in 0..SCREEN_WIDTH {
                        let src = self.video_buffer[src_offset] as usize;

                        let r = self.pallets[src * 3];
                        let g = self.pallets[src * 3 + 1];
                        let b = self.pallets[src * 3 + 2];

                        for m_h in 0..self.mul_height {
                            for m_w in 0..self.mul_width {
                                let dst_offset = (((y * self.mul_height) + m_h) * pitch)
                                    + ((x * self.mul_width) + m_w) * 3; // 3 = pixel width
                                buffer[dst_offset] = r;
                                buffer[dst_offset + 1] = g;
                                buffer[dst_offset + 2] = b;
                            }
                        }

                        src_offset += 1;
                    }
                }
            })
            .expect("with_lock");

        self.canvas.clear();
        self.canvas.copy(&self.texture, None, None).expect("copy");
        self.canvas.present();

        self.handle_keys();
    }

    pub fn set_buffer(&mut self, dest: usize, v: u8) {
        self.video_buffer[dest] = v;
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
