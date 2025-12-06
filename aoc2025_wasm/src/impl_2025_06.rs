use anyhow::{Context, Result};
use ndarray::s;

use crate::grid_util::make_byte_grid;

fn calculate_p1(inp: &str) -> Result<u64> {
    let data = inp
        .lines()
        .map(|line| {
            line.trim()
                .split(" ")
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let y_size = data.len();
    let x_size = data.first().context("not enough data")?.len();
    let mut ans = 0;

    for x in 0..x_size {
        let op = *data
            .get(y_size - 1)
            .context("not enough data")?
            .get(x)
            .context("not enough data")?;

        ans += (0..(y_size - 1))
            .filter_map(|y| data.get(y).and_then(|v| v.get(x)))
            .filter_map(|itm| itm.parse::<u64>().ok())
            .fold(if op == "+" { 0 } else { 1 }, |a, b| {
                if op == "+" { a + b } else { a * b }
            });
    }

    Ok(ans)
}

fn calculate_p2(inp: &str) -> Result<u64> {
    let grid = make_byte_grid(inp)?;

    let mut op = b'+';
    let mut cumulative = 0;
    let mut ans = 0;

    for x in 0..grid.dim().1 {
        if let Some(&o) = grid.get((grid.dim().0 - 1, x))
            && o != b' '
        {
            op = o;
            ans += cumulative;
            if op == b'+' {
                cumulative = 0;
            } else {
                cumulative = 1;
            }
        }

        let n = grid
            .slice(s![..-1, x])
            .iter()
            .filter(|elem| elem.is_ascii_digit())
            .map(|&elem| (elem - b'0') as u64)
            .fold(0, |a, b| a * 10 + b);

        if n != 0 {
            if op == b'+' {
                cumulative += n;
            } else {
                cumulative *= n;
            }
        }
    }

    Ok(ans + cumulative)
}

pub fn run_2025_06(inp: &str) -> Result<String> {
    let p1 = calculate_p1(inp)?;
    let p2 = calculate_p2(inp)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_06");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_06");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate_p1(EXAMPLE_DATA).unwrap(), 4277556);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate_p2(EXAMPLE_DATA).unwrap(), 3263827);
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(calculate_p1(REAL_DATA).unwrap(), 5524274308182);
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(calculate_p2(REAL_DATA).unwrap(), 8843673199391);
    }
}
