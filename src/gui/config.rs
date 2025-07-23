#![allow(non_camel_case_types)]

type coord = u16;

/// screen width (mobile phone emulation)
pub const W: coord = 240;
/// screen height (mobile phone emulation)
pub const H: coord = 320;

/// root window background
pub const root_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x22, 0x22, 0x22);
// status bar background
pub const status_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x22, 0x11, 0x11);
// navigator bar background
pub const navbar_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x11, 0x11, 0x22);
