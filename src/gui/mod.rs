#![allow(non_snake_case)]

mod config;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::rwops::{self, RWops};
use sdl2::ttf;

use std::fmt::Write;
use std::time::SystemTime;

use crate::gui::config::font_size;

pub struct GUI {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    fps: std::time::Duration,
    status: Rect,
    navbar: Rect,
    ttf_context: &'static sdl2::ttf::Sdl2TtfContext,
    font: &'static ttf::Font<'static, 'static>,
}

const SpaceMono_Regular: &[u8] = include_bytes!("../../static/font/SpaceMono-Regular.ttf");

impl GUI {
    pub fn init(title: &str) -> Self {
        let iW = config::W as i32;
        let iH = config::H as i32;
        let uW = config::W as u32;
        let uH = config::H as u32;
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, uW, uH)
            .position(iW / 8, iH * 2)
            // .position_centered()
            // .opengl()
            // .flags(WindowFlags::INPUT_FOCUS)
            .borderless()
            // .fullscreen_desktop()
            .build()
            .unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let canvas = window.clone().into_canvas().build().unwrap();
        let fps = std::time::Duration::new(1, 0); //1_000_000_000u32 / 60);
        let status = Rect::new(0, 0, uW, font_size as u32);
        let navbar = Rect::new(0, iH / 0x10 * 0x0F, uW, font_size as u32);
        let ttf_context = {
            let ttf_context = Box::new(sdl2::ttf::init().unwrap());
            Box::leak(ttf_context)
        };
        let font = {
            let rw = rwops::RWops::from_bytes(SpaceMono_Regular).unwrap();
            let font = Box::new(
                ttf_context
                    .load_font_from_rwops(rw, config::font_size / 10 * 8)
                    .unwrap(),
            );
            Box::leak(font)
        };
        Self {
            sdl_context,
            window,
            event_pump,
            canvas,
            fps,
            status,
            navbar,
            ttf_context,
            font,
        }
    }

    /// stop SDL session
    pub fn fini(&self) {}

    /// event loop
    pub fn run(&mut self) {
        'main: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main,
                    _ => {}
                }
            }
            self.render();
            ::std::thread::sleep(self.fps);
        }
    }

    /// render loop
    pub fn render(&mut self) {
        // root window
        self.canvas.set_draw_color(config::root_bg);
        self.canvas.clear();
        // status bar
        self.canvas.set_draw_color(config::status_bg);
        self.canvas.fill_rect(self.status).unwrap();
        // clock
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let seconds = now.as_secs();
        // render now string
        let hours = (seconds / 3600) % 24;
        let minutes = (seconds / 60) % 60;
        let seconds = seconds % 60;
        let mut time_str = String::new();
        write!(&mut time_str, "{:02}:{:02}:{:02}", hours, minutes, seconds).unwrap();
        let clock_surface = self
            .font
            .render(&time_str)
            .blended(config::status_clock)
            .unwrap();
        let clock_texture = self
            .canvas
            .create_texture_from_surface(&clock_surface)
            .unwrap();
        let clock_query = clock_texture.query();
        let w = clock_query.width;
        let h = clock_query.height;
        let x = self.status.w - w as i32;
        let clock_rect = Rect::new(x, 0, w, h);
        self.canvas.copy(&clock_texture, None, clock_rect).unwrap();
        // status bar
        self.canvas.set_draw_color(config::navbar_bg);
        self.canvas.fill_rect(self.navbar).unwrap();
        // repaint
        self.canvas.present();
    }
}
