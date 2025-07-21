#![allow(unused_variables)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc = argv.len();
    arg(0, &argv[0]);
    for (i, argv) in argv.iter().skip(1).enumerate() {
        arg(i + 1, argv);
    }
}

fn arg(argc: usize, argv: &str) {
    eprintln!("argv[{argc}] = {argv:?}");
}
