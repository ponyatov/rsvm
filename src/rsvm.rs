#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

/// main memory size, bytes
pub const Msz: usize = 0x10000;
/// return stack size, cells
pub const Rsz: usize = 0x100;
/// data stack size
pub const Dsz: usize = 0x10;

/// memory addresses should be short for smaller command arguments
pub type Addr = u16;

/// primitive types can be stored in D
#[derive(Clone, Copy)]
pub union Cell {
    /// integer
    i: i32,
    /// floating point
    f: f32,
    /// char
    c: char,
    /// boolean
    b: bool,
    /// nil
    n: (),
    /// memory address
    a: Addr,
}

/// data stack
static mut D: [Cell; Dsz] = [Cell { n: () }; Dsz];
/// data stack pointer
static mut Dp: usize = 0;

/// return stack
static mut R: [usize; Rsz] = [0; Rsz];
static mut Rp: usize = 0;
