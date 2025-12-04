use anyhow::Result;
use ndarray::Array2;

fn neighbour_coords(y: usize, x: usize) -> [(usize, usize); 8] {
    [
        (y.wrapping_sub(1), x.wrapping_sub(1)),
        (y.wrapping_sub(1), x),
        (y.wrapping_sub(1), x.wrapping_add(1)),
        (y, x.wrapping_sub(1)),
        (y, x.wrapping_add(1)),
        (y.wrapping_add(1), x.wrapping_sub(1)),
        (y.wrapping_add(1), x),
        (y.wrapping_add(1), x.wrapping_add(1)),
    ]
}

fn neighbours(grid: &Array2<bool>, y: usize, x: usize) -> usize {
    neighbour_coords(y, x)
        .into_iter()
        .filter(|&(y, x)| *grid.get((y, x)).unwrap_or(&false))
        .count()
}

fn calculate(mut grid: Array2<bool>) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut changed_q = Vec::with_capacity(2048);

    for y in 0..grid.dim().0 {
        for x in 0..grid.dim().1 {
            if *grid.get((y, x)).expect("indexes are valid") && neighbours(&grid, y, x) < 4 {
                p1 += 1;
                changed_q.push((y, x));
            }
        }
    }

    while let Some((y, x)) = changed_q.pop() {
        {
            let itm = grid.get_mut((y, x)).expect("indexes are valid");
            if !*itm {
                continue;
            }
            *itm = false
        }
        p2 += 1;

        neighbour_coords(y, x)
            .into_iter()
            .filter(|(y, x)| *grid.get((*y, *x)).unwrap_or(&false) && neighbours(&grid, *y, *x) < 4)
            .for_each(|(y, x)| changed_q.push((y, x)));
    }

    (p1, p2)
}

pub fn run_2025_04(inp: &str) -> Result<String> {
    let grid = crate::grid_util::make_bool_grid::<b'@'>(inp)?;
    let (p1, p2) = calculate(grid);
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_04");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_04");

    #[test]
    fn test_example() {
        let grid = crate::grid_util::make_bool_grid::<b'@'>(EXAMPLE_DATA).unwrap();
        assert_eq!(calculate(grid), (13, 43));
    }

    #[test]
    fn test_real() {
        let grid = crate::grid_util::make_bool_grid::<b'@'>(REAL_DATA).unwrap();
        assert_eq!(calculate(grid), (1349, 8277));
    }
}
