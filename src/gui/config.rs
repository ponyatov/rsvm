#![allow(non_camel_case_types)]

type coord = u16;

/// screen width (mobile phone emulation)
pub const W: coord = 240;
/// screen height (mobile phone emulation)
pub const H: coord = 320;

/// root window background
pub const root_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x22, 0x22, 0x33);
// status bar background
pub const status_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x22, 0x11, 0x11);
pub const status_clock: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x12, 0x34, 0x56);
// navigator bar background
pub const navbar_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x11, 0x22, 0x11);

/// base font size
pub const font_size: u8 = 10;
