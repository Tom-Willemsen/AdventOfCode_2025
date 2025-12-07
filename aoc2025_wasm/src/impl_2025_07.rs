use anyhow::{Context, Result};
use ndarray::{Array2, s};

use crate::grid_util::make_byte_grid;

fn calculate(mut grid: Array2<u8>) -> Option<(usize, u64)> {
    let mut cache = Array2::from_elem(grid.dim(), 0);

    grid.indexed_iter_mut()
        .find(|(_, elem)| elem == &&b'S')
        .map(|((y, x), _)| *cache.get_mut((y, x)).expect("will exist") = 1)?;

    let mut p1 = 0;
    for y in 1..grid.dim().0 {
        for x in 0..grid.dim().1 {
            let up = *grid.get((y - 1, x))?;
            if up == b'|' || up == b'S' {
                let here = *grid.get((y, x))?;
                if here == b'^' {
                    p1 += 1;
                    *cache.get_mut((y, x - 1))? += *cache.get((y - 1, x))?;
                    *cache.get_mut((y, x + 1))? += *cache.get((y - 1, x))?;
                    *grid.get_mut((y, x - 1))? = b'|';
                    *grid.get_mut((y, x + 1))? = b'|';
                } else {
                    *grid.get_mut((y, x))? = b'|';
                    *cache.get_mut((y, x))? += *cache.get((y - 1, x))?;
                }
            }
        }
    }

    let p2 = cache.slice(s![cache.dim().0 - 1, ..]).sum();

    Some((p1, p2))
}

pub fn run_2025_07(inp: &str) -> Result<String> {
    let grid = make_byte_grid(inp)?;
    let (p1, p2) = calculate(grid).context("no solution (bad input?)")?;
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
            calculate(make_byte_grid(EXAMPLE_DATA).unwrap()).unwrap().0,
            21
        );
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(
            calculate(make_byte_grid(EXAMPLE_DATA).unwrap()).unwrap().1,
            40
        );
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(
            calculate(make_byte_grid(REAL_DATA).unwrap()).unwrap().0,
            1711
        );
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(
            calculate(make_byte_grid(REAL_DATA).unwrap()).unwrap().1,
            36706966158365
        );
    }
}
