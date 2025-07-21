#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

const Msz: usize = 0x10000;
/// memory size
const Rsz: usize = 0x100;
/// return stack size
const Dsz: usize = 0x10;
/// data stack size

/// primitive types can be stored in D
#[derive(Clone, Copy)]
union Primitive {
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
}

/// data stack
static mut D: [Primitive; Dsz] = [Primitive { n: () }; Dsz];
/// data stack pointer
static mut Dp: usize = 0;

/// return stack
static mut R: [usize; Rsz] = [0; Rsz];
static mut Rp: usize = 0;
