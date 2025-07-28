//! executable file integrates VM components
//! - parser
//! - byte-code compiler (in-memory only)
//! - byte-code interpreter

mod config;
mod gui;

use memmap2::Mmap;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::Write;

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let _argc = argv.len();
    arg(0, &argv[0]);
    for (argc, argv) in argv.iter().enumerate().skip(1) {
        arg(argc, argv);
        let file = File::open(Path::new(argv)).unwrap();
        let src = unsafe { Mmap::map(&file).unwrap() };
        eprintln!("File size: {} bytes", src.len());
        // eprintln!("{:?}", &mmap[..] as &str);
        io::stdout().write_all(&src[..]).unwrap();
    }
    gui::GUI::new(&argv[0]).run();
}

fn arg(argc: usize, argv: &str) {
    eprintln!("argv[{argc}] = {argv:?}");
}
