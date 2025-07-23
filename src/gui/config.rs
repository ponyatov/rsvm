#![allow(non_camel_case_types)]

type coord = i16;

/// screen width (mobile phone emulation)
pub const W: coord = 240;
/// screen height (mobile phone emulation)
pub const H: coord = 320;

/// root window background
pub const root_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x22, 0x22, 0x33);
// status bar background
pub const status_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x22, 0x11, 0x11);
pub const status_clock: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0xAA, 0xBB, 0xCC);
// navigator bar background
pub const navbar_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x11, 0x22, 0x11);

/// base font size
pub const font_size: u16 = H as u16 / 0x10;
