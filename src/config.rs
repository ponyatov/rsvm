//! VM configuration parameters
#![allow(dead_code)]
// #![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

/// max VM memory size
pub const VM_MSZ: usize = 0x10000;
/// return stack size (max call depth)
pub const VM_RSZ: usize = 0x100;
/// data stack size (limited)
pub const VM_DSZ: usize = 0x10;

/// Web server default bind ip
pub mod server {
    pub const ip: &str = "127.0.0.1";
    // pub const IP: &str = "0.0.0.0";
    /// Web server IP port
    pub const port: u16 = 12345;
    /// bind address constant
    pub const bind: &str = const_format::formatcp!("{ip}:{port}");
}

/// screen width
pub mod gui {
    /// screen width (mobile phone emulation)
    pub const width: u16 = 240;
    /// screen height (mobile phone emulation)
    pub const height: u16 = 320;
    /// base font size
    pub const font_size: u16 = height / 4;
    /// large icons (for tiny phone screen)
    pub const icon_size: u16 = 64;

    // /// root window background
    pub const root_bg: (u8, u8, u8) = (0x22, 0x22, 0x22);
    // // status bar background
    // pub const status_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x22, 0x11, 0x11);
    // pub const status_clock: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0xAA, 0xBB, 0xCC);
    // // navigator bar background
    // pub const navbar_bg: sdl2::pixels::Color = sdl2::pixels::Color::RGB(0x11, 0x22, 0x11);
}
