#![allow(non_snake_case)]

mod config;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct GUI {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    event_pump: sdl2::EventPump,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    fps: std::time::Duration,
}

impl GUI {
    /// start SDL session
    pub fn init(title: &str) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window(title, config::W.into(), config::H.into())
            .position((config::W / 8).into(), (config::H * 2).into())
            // .position_centered()
            // .opengl()
            // .flags(WindowFlags::INPUT_FOCUS)
            .borderless()
            // .fullscreen_desktop()
            .build()
            .unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let canvas = window.clone().into_canvas().build().unwrap();
        let fps = std::time::Duration::new(0, 1_000_000_000u32 / 60);
        Self {
            sdl_context,
            window,
            event_pump,
            canvas,
            fps,
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
        self.canvas.set_draw_color(config::background);
        self.canvas.clear();
        self.canvas.present();
    }
}
