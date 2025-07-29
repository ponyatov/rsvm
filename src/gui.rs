//! generic GUI library for framebuffer-like output

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::config::*;

/// 2D coordinates
struct Coord(i16, i16);

/// color: RGB TrueColor
struct Color(u8, u8, u8);

#[cfg(feature = "sdl")]
pub struct GUI<'a> {
    title: &'a str,
    sdl_context: sdl2::Sdl,
}

impl<'a> GUI<'a> {
    pub fn new(title: &'a str) -> Self {
        GUI {
            title,
            sdl_context: sdl2::init().unwrap(),
        }
    }

    #[cfg(feature = "sdl")]
    pub fn run(&mut self) {}
}
