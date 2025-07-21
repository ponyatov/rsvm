#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

const Msz: usize = 0x10000;
const Rsz: usize = 0x100;
const Dsz: usize = 0x10;

union Primitive {
    i: i32,  // integer
    f: f32,  // floating point
    c: char, // char
    b: bool, // boolean
}
