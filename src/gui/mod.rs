#![allow(non_snake_case)]

mod config;

/// start SDL session
pub fn init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let wMain = video_subsystem
        .window("SDL2", config::W, config::H)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
}

/// stop SDL session
pub fn fini() {}
