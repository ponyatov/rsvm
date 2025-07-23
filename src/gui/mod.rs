#![allow(non_snake_case)]

mod config;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct GUI {
    sdl_context: sdl2::Sdl,
    wMain: sdl2::video::Window,
    event_pump: sdl2::EventPump,
}

impl GUI {
    /// start SDL session
    pub fn init(title: &str) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let wMain = video_subsystem
            .window(title, config::W.into(), config::H.into())
            .position((config::W / 8).into(), (config::H * 2).into())
            // .position_centered()
            // .opengl()
            .build()
            .unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        Self {
            sdl_context,
            wMain,
            event_pump,
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
        }
    }
}
