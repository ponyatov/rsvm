//! generic GUI library for framebuffer-like output

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::config::gui::font_size;
use crate::config::*;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::surface::{self, Surface};
use sdl2::video::{Window, WindowContext};
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
    image_context: sdl2::image::Sdl2ImageContext,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    canvas: Canvas<Window>,
    status_rect: sdl2::rect::Rect,
    navbar_rect: sdl2::rect::Rect,
    logo_rect: sdl2::rect::Rect,
}

// #[cfg(feature = "sdl")]
impl<'a> GUI<'a> {
    pub fn new(title: &'a str) -> Self {
        use crate::config;

        let sdl_context = sdl2::init().unwrap();
        let image_context = sdl2::image::init(InitFlag::PNG).unwrap();
        let video_subsystem = sdl_context.clone().video().unwrap();
        let display_mode = video_subsystem.current_display_mode(0).unwrap();
        let window = video_subsystem
            .window(title, config::gui::width as u32, config::gui::height as u32)
            .always_on_top()
            .borderless()
            .position(
                (config::gui::icon_size / 4).into(),
                display_mode.h - (config::gui::height + 2 * config::gui::icon_size) as i32,
            )
            .build()
            .unwrap();
        let canvas = window.clone().into_canvas().build().unwrap();
        let status_rect = sdl2::rect::Rect::new(
            0,
            0,
            config::gui::width as u32,
            config::gui::font_size as u32,
        );
        let navbar_rect = sdl2::rect::Rect::new(
            0,
            (config::gui::height - config::gui::font_size).into(),
            config::gui::width as u32,
            config::gui::font_size as u32,
        );
        let logo_rect = sdl2::rect::Rect::new(
            (config::gui::font_size / 4).into(),
            (config::gui::font_size / 4).into(),
            config::gui::icon_size as u32,
            config::gui::icon_size as u32,
        );
        GUI {
            title,
            sdl_context,
            image_context,
            video_subsystem,
            window,
            canvas,
            status_rect,
            navbar_rect,
            logo_rect,
        }
    }

    pub fn paint(&mut self) {
        use crate::config;

        // background
        let (r, g, b) = config::gui::root_bg;
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        self.canvas.clear();

        // statusbar
        let (r, g, b) = config::gui::status_bg;
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        self.canvas.fill_rect(self.status_rect).unwrap();

        // logo
        let texture_creator = self.canvas.texture_creator();
        let logo_texture = texture_creator.load_texture_bytes(LOGO_PNG).unwrap();
        self.canvas
            .copy(&logo_texture, None, self.logo_rect)
            .unwrap();

        // navbar
        let (r, g, b) = config::gui::navbar_bg;
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        self.canvas.fill_rect(self.navbar_rect).unwrap();

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
