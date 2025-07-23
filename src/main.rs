#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(special_module_name)]

mod lib;
use lib::*;

mod gui;
use gui::*;

use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    arg(0, &argv[0]);
    for (i, argv) in argv.iter().skip(1).enumerate() {
        arg(i + 1, argv);
        let srcfile = File::open(Path::new(argv)).unwrap();
        let src = unsafe { Mmap::map(&srcfile).unwrap() };
        eprintln!("File size: {} bytes", src.len());
    }
    eprintln!("cell:{:?}", size_of::<prim>());
    gui::GUI::init(&argv[0]).run();
}

fn arg(argc: usize, argv: &str) {
    eprintln!("argv[{argc}] = {argv:?}");
}
