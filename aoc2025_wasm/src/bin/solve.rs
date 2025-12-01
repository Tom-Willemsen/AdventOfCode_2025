use aoc2025_wasm::run_day;
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    pub day: u32,

    #[clap(short, long)]
    pub input: String,
}

fn main() {
    let args = Cli::parse();
    let inp = fs::read_to_string(args.input).expect("can't open input file");

    println!("{}", run_day(args.day, &inp));
}
