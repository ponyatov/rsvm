//! generic GUI library for framebuffer-like output

#![allow(dead_code)]
#![allow(unused_imports)]

use sdl2::render::Canvas;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use crate::config::*;

/// 2D coordinates
struct Coord(i16, i16);

/// color: RGB TrueColor
struct Color(u8, u8, u8);

// #[cfg(feature = "sdl")]
pub struct GUI<'a> {
    title: &'a str,
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    // texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    // texture: sdl2::render::Texture<'a>,
}

// #[cfg(feature = "sdl")]
impl<'a> GUI<'a> {
    pub fn new(title: &'a str) -> Self {
        use crate::config;

        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.clone().video().unwrap();
        let window = video_subsystem
            .window(title, config::gui::width as u32, config::gui::height as u32)
            .build()
            .unwrap();
        let canvas = window.clone().into_canvas().build().unwrap();
        GUI {
            title,
            sdl_context,
            video_subsystem,
            window,
            canvas,
        }
    }

    pub fn paint(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0x22, 0x22, 0x22));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            self.paint();
            for event in event_pump.poll_iter() {
                match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                sdl2::event::Event::Quit { .. } => break 'running,
                _ => {}
            }
        }}
    }
}
