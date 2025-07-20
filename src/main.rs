#![allow(unused_variables)]

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    arg(0, &argv[0]);
}

fn arg(argc: usize, argv: &str) {
    eprintln!("argv[{argc}] = {argv:?}");
}
