#![allow(non_camel_case_types)]

type coord = u16;

/// screen width (mobile phone emulation)
pub const W: coord = 240;
/// screen height (mobile phone emulation)
pub const H: coord = 320;

/// root window background
pub const background: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x22, 0x22, 0x22);
