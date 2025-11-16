use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_2025_01(inp: &str) -> String {
    inp.lines()
        .filter_map(|x| x.parse::<i64>().ok())
        .sum::<i64>()
        .to_string()
}
