mod caves;
mod game;
mod graphics;
mod input;

mod prelude {
    pub use crate::game::Game;
    pub use sdl2::event::Event;
    pub use sdl2::image::{InitFlag, LoadTexture};
    pub use sdl2::keyboard::Keycode;
    pub use sdl2::pixels::PixelFormatEnum;
    pub use sdl2::rect::Rect;

    pub use crate::input::Keys;

    pub const BORDER: u32 = 20;
    pub const ZOOM: u32 = 3;

    pub const MAP_WIDTH: usize = 40;
    pub const MAP_HEIGHT: usize = 22;
    pub const NUM_TILES: usize = MAP_WIDTH*MAP_HEIGHT;
}

use prelude::*;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG)?;

    let window = video_subsystem
        .window(
            "Boulder Dash",
            320 * ZOOM + 2 * BORDER,
            200 * ZOOM + 2 * BORDER,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(sdl2::pixels::Color::RGBA(0, 0, 0, 255));

    let texture_creator = canvas.texture_creator();

    let texture_titolo = texture_creator.load_texture("assets/titolo.png")?;
    let texture_sfondo_titolo = texture_creator.load_texture("assets/sfondo_titolo.png")?;

    let mut graphics_set = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, 128 * ZOOM, 256 * ZOOM)
        .map_err(|e| e.to_string())?;
    // Create the graphics
    graphics_set.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        let cols = [
            Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
            Color {
                r: 124,
                g: 124,
                b: 124,
                a: 255,
            },
            Color {
                r: 140,
                g: 84,
                b: 24,
                a: 255,
            },
            Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            },
        ];
        for y in 0..256 {
            for x in 0..16 {
                let b = graphics::data(y * 16 + x);
                let b1 = (b >> 6) as usize;
                let b2 = ((b >> 4) & 3) as usize;
                let b3 = ((b >> 2) & 3) as usize;
                let b4 = (b & 3) as usize;
                put_pixel(buffer, pitch, x * 4, y, &cols[b1]);
                put_pixel(buffer, pitch, x * 4 + 1, y, &cols[b2]);
                put_pixel(buffer, pitch, x * 4 + 2, y, &cols[b3]);
                put_pixel(buffer, pitch, x * 4 + 3, y, &cols[b4]);
            }
        }
    })?;

    let mut event_pump = sdl_context.event_pump()?;

    let mut game = Game::new();

    game.textures.push(texture_titolo);
    game.textures.push(texture_sfondo_titolo);
    game.textures.push(graphics_set);

    game.set_intro_state();

    while game.running {
        let now = Instant::now();
        game.get_input(&mut event_pump);
        game.update();
        game.render(&mut canvas);
        let elapsed = now.elapsed().as_millis() as u64;
        if elapsed < 16 {
            std::thread::sleep(Duration::from_millis(16 - elapsed));
        }
    }

    Ok(())
}

fn put_pixel(buffer: &mut [u8], pitch: usize, x: usize, y: usize, col: &Color) {
    let ex = x * (2 * ZOOM as usize);
    let ey = y * (ZOOM as usize);
    for dy in 0..(ZOOM as usize) {
        for dx in 0..(2 * ZOOM as usize) {
            let offset = (ey + dy) * pitch + (ex + dx) * 4;
            buffer[offset] = col.r;
            buffer[offset + 1] = col.g;
            buffer[offset + 2] = col.b;
            buffer[offset + 3] = col.a;
        }
    }
}
