#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

/// main memory size, bytes
pub const Msz: usize = 0x10000;
/// return stack size, cells
pub const Rsz: usize = 0x100;
/// data stack size
pub const Dsz: usize = 0x10;

/// primitive types can be stored in D
#[derive(Clone, Copy)]
pub union prim {
    /// integer
    i: i32,
    /// floating point
    f: f32,
    /// char
    c: char,
    /// boolean
    b: bool,
    /// generic pointer
    p: *mut prim,
    /// vm command `fn () -> ()`
    cmd: fn(),
}

/// data stack
static mut D: [prim; Dsz] = [prim { i: 0 }; Dsz];
/// data stack pointer
static mut Dp: usize = 0;

/// return stack
static mut R: [usize; Rsz] = [0; Rsz];
/// return stack pointer
static mut Rp: usize = 0;

use std::sync::atomic::{AtomicBool, Ordering};

/// tracing flag
static trace_: AtomicBool = AtomicBool::new(true);

/// print command run into trace
fn trace(command: &str) {
    if trace_.load(Ordering::Relaxed) {
        eprintln!("nop");
    }
}

/// `( -- )` do nothing
fn nop() {
    trace("nop");
}

/// `( -- )` stop system
fn halt() {
    trace("halt");
    std::process::exit(0);
}
