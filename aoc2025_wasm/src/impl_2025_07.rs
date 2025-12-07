use anyhow::{Context, Result};
use ndarray::Array2;

use crate::grid_util::make_byte_grid;

fn many_worlds(grid: &Array2<u8>, mut y: usize, x: usize, cache: &mut Array2<u64>) -> u64 {
    while grid.get((y + 1, x)) == Some(&b'.') {
        y += 1;
    }

    if y >= grid.dim().0 - 1 {
        return 1;
    }

    if let Some(cached_result) = cache.get((y, x))
        && cached_result != &0
    {
        return *cached_result;
    }

    let result = many_worlds(grid, y + 1, x - 1, cache) + many_worlds(grid, y + 1, x + 1, cache);
    if let Some(cached_result) = cache.get_mut((y, x)) {
        *cached_result = result;
    }
    result
}

fn calculate(grid: &Array2<u8>) -> Result<(usize, u64)> {
    let mut cache = Array2::from_elem(grid.dim(), 0);

    let p2 = grid
        .indexed_iter()
        .find(|&(_, elem)| elem == &b'S')
        .map(|((y, x), _)| many_worlds(grid, y, x, &mut cache))
        .context("can't find start")?;

    let p1 = cache.into_iter().filter(|&n| n > 0).count();

    Ok((p1, p2))
}

pub fn run_2025_07(inp: &str) -> Result<String> {
    let grid = make_byte_grid(inp)?;
    let (p1, p2) = calculate(&grid)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_07");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_07");

    #[test]
    fn test_example_p1() {
        assert_eq!(
            calculate(&make_byte_grid(EXAMPLE_DATA).unwrap()).unwrap().0,
            21
        );
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(
            calculate(&make_byte_grid(EXAMPLE_DATA).unwrap()).unwrap().1,
            40
        );
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(
            calculate(&make_byte_grid(REAL_DATA).unwrap()).unwrap().0,
            1711
        );
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(
            calculate(&make_byte_grid(REAL_DATA).unwrap()).unwrap().1,
            36706966158365
        );
    }
}
