use anyhow::{Context, Result};
use rayon::prelude::*;

fn fibonacci(x: i64) -> i64 {
    match x {
        0 => 0,
        1 => 1,
        _ => fibonacci(x - 1) + fibonacci(x - 2),
    }
}

pub fn run_2025_01(inp: &str) -> Result<String> {
    let nums: Vec<i64> = inp
        .lines()
        .map(|x| x.parse::<i64>().with_context(|| "failed to parse int"))
        .collect::<Result<_>>()?;

    Ok(nums
        .into_par_iter()
        .map(|n| fibonacci(n))
        .sum::<i64>()
        .to_string())
}
