//! generic GUI library for framebuffer-like output

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::config::*;

/// 2D coordinates
type Coord = (i16, i16);

/// color: RGB TrueColor
type Color = (u8, u8, u8);

pub struct GUI<'a> {
    title: &'a str,
}

impl<'a> GUI<'a> {
    pub fn new(title: &'a str) -> Self {
        GUI { title }
    }

    pub fn run(&self) {}
}
