//! generic GUI library for framebuffer-like output

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::config::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::surface::{self, Surface};
use std::time::Duration;

/// 2D coordinates
struct Coord(i16, i16);

/// color: RGB TrueColor
struct Color(u8, u8, u8);

pub const LOGO_PNG: &[u8] = include_bytes!("../doc/logo.png");

// #[cfg(feature = "sdl")]
pub struct GUI<'a> {
    title: &'a str,
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    status_rect: sdl2::rect::Rect,
    logo_texture: sdl2::render::Texture<'a>,
    logo_rect: sdl2::rect::Rect,
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
        let canvas = window.into_canvas().build().unwrap();
        let status_rect = sdl2::rect::Rect::new(
            (config::gui::font_size / 2).into(),
            (config::gui::font_size / 2).into(),
            config::gui::width as u32,
            config::gui::font_size as u32,
        );
        let texture_creator = canvas.texture_creator();
        let mut rwops_logo = sdl2::rwops::RWops::from_bytes(LOGO_PNG).unwrap();
        let logo_surface = sdl2::surface::Surface::load_png_rw(&mut rwops_logo).unwrap();
        let logo_texture = texture_creator
            .create_texture_from_surface(&logo_surface)
            .unwrap();
        let logo_rect = sdl2::rect::Rect::new(
            config::gui::font_size as i32,
            config::gui::font_size as i32,
            config::gui::icon_size as u32,
            config::gui::icon_size as u32,
        );
        GUI {
            title,
            sdl_context,
            video_subsystem,
            window,
            canvas,
            status_rect,
            logo_texture,
            logo_rect,
        }
    }

    pub fn paint(&mut self) {
        use crate::config;

        // background
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(config::gui::root_bg));
        self.canvas.clear();

        // statusbar
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0x22, 0x11, 0x11));
        self.canvas.fill_rect(status_rect).unwrap();

        // logo
        self.canvas
            .copy(&self.logo_texture, None, self.logo_rect)
            .unwrap();

        // show
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
                    _ => {}
                }
            }
            ::std::thread::sleep(Duration::new(1, 0));
            // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
