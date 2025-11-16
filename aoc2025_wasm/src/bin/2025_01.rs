use aoc2025_wasm::{impl_2025_01::run_2025_01, Cli, Parser};
use std::fs;

fn main() {
    let args = Cli::parse();
    let inp = fs::read_to_string(args.input).expect("can't open input file");

    println!("{}", run_2025_01(&inp));
}
