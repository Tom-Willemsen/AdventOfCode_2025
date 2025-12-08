pub(crate) mod grid_util;
mod impl_2025_01;
mod impl_2025_02;
mod impl_2025_03;
mod impl_2025_04;
mod impl_2025_05;
mod impl_2025_06;
mod impl_2025_07;
mod impl_2025_08;

pub use wasm_bindgen_rayon::init_thread_pool;

use anyhow::anyhow;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_day(day: u32, input: &str) -> String {
    let res = match day {
        1 => impl_2025_01::run_2025_01(input),
        2 => impl_2025_02::run_2025_02(input),
        3 => impl_2025_03::run_2025_03(input),
        4 => impl_2025_04::run_2025_04(input),
        5 => impl_2025_05::run_2025_05(input),
        6 => impl_2025_06::run_2025_06(input),
        7 => impl_2025_07::run_2025_07(input),
        8 => impl_2025_08::run_2025_08(input),
        _ => Err(anyhow!("No solution for that day yet")),
    };

    res.unwrap_or_else(|e| format!("Error: {}", e))
}
