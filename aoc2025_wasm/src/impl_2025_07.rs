use std::collections::HashMap;

use anyhow::{Context, Result};
use ndarray::Array2;

use crate::grid_util::make_byte_grid;

fn calculate_p1(mut grid: Array2<u8>) -> u64 {
    let mut ans = 0;
    for y in 1..grid.dim().0 {
        for x in 0..grid.dim().1 {
            if grid.get((y - 1, x)) == Some(&b'|') || grid.get((y - 1, x)) == Some(&b'S') {
                if grid.get((y, x)) == Some(&b'.') {
                    *grid.get_mut((y, x)).expect("invalid indices") = b'|';
                } else if grid.get((y, x)) == Some(&b'^') {
                    ans += 1;
                    if let Some(left) = grid.get_mut((y, x - 1)) {
                        *left = b'|';
                    }
                    if let Some(right) = grid.get_mut((y, x + 1)) {
                        *right = b'|';
                    }
                }
            }
        }
    }
    ans
}

fn many_worlds(
    grid: &Array2<u8>,
    mut y: usize,
    x: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(cached_result) = cache.get(&(y, x)) {
        return *cached_result;
    }

    let original_y = y;
    while grid.get((y + 1, x)) == Some(&b'.') {
        y += 1;
    }

    if y >= grid.dim().0 - 1 {
        return 1;
    }

    let result = many_worlds(grid, y + 1, x - 1, cache) + many_worlds(grid, y + 1, x + 1, cache);
    cache.insert((original_y, x), result);
    result
}

fn calculate_p2(grid: &Array2<u8>) -> Result<u64> {
    let mut cache = HashMap::<(usize, usize), u64>::default();
    grid.indexed_iter()
        .find(|&(_, elem)| elem == &b'S')
        .map(|((y, x), _)| many_worlds(grid, y, x, &mut cache))
        .context("can't find start")
}

pub fn run_2025_07(inp: &str) -> Result<String> {
    let grid = make_byte_grid(inp)?;
    let p2 = calculate_p2(&grid)?;
    let p1 = calculate_p1(grid);
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_07");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_07");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate_p1(make_byte_grid(EXAMPLE_DATA).unwrap()), 21);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(
            calculate_p2(&make_byte_grid(EXAMPLE_DATA).unwrap()).unwrap(),
            40
        );
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(calculate_p1(make_byte_grid(REAL_DATA).unwrap()), 1711);
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(
            calculate_p2(&make_byte_grid(REAL_DATA).unwrap()).unwrap(),
            36706966158365
        );
    }
}
