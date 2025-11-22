use aoc2025_wasm::{Cli, Parser, impl_2025_01::run_2025_01};
use std::fs;

fn main() {
    let args = Cli::parse();
    let inp = fs::read_to_string(args.input).expect("can't open input file");

    println!(
        "{}",
        run_2025_01(&inp).unwrap_or_else(|e| format!("Error: {e}"))
    );
}
