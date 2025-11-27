pub mod impl_2025_01;

pub use wasm_bindgen_rayon::init_thread_pool;

pub use clap::Parser;

use anyhow::anyhow;
use wasm_bindgen::prelude::*;

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub input: String,
}

#[wasm_bindgen]
pub fn run_day(day: u32, input: &str) -> String {
    let res = match day {
        1 => impl_2025_01::run_2025_01(input),
        _ => Err(anyhow!("No solution for that day yet")),
    };

    res.unwrap_or_else(|e| format!("Error: {}", e))
}
