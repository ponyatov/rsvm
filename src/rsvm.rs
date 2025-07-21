#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use const_format::str_splice_out;

/// main memory size, bytes
pub const Msz: usize = 0x10000;
/// return stack size, cells
pub const Rsz: usize = 0x100;
/// data stack size
pub const Dsz: usize = 0x10;

/// memory addresses should be short for smaller command arguments
pub type addr = u16;
/// memory region size also limited
pub type Size = u16;
/// chars must be encoded in UCS2 able to work with CJK & Cyrillic text data
pub type ucs2 = u16;
/// slice pair
#[repr(C)]
#[derive(Clone, Copy)]
pub struct slice {
    addr: addr,
    size: Size,
}

/// primitive types can be stored in D
#[derive(Clone, Copy)]
pub union prim {
    /// integer
    i: i32,
    /// floating point
    f: f32,
    /// char
    c: ucs2,
    /// boolean
    b: bool,
    /// nil
    n: (),
    /// memory address
    a: addr,
    /// low-memory slice
    s: slice,
}

/// data stack
static mut D: [prim; Dsz] = [prim { n: () }; Dsz];
/// data stack pointer
static mut Dp: usize = 0;

/// return stack
static mut R: [usize; Rsz] = [0; Rsz];
static mut Rp: usize = 0;
