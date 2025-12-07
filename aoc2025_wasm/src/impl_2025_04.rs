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

fn neighbours(grid: &Array2<bool>, y: usize, x: usize) -> u8 {
    if grid.get((y, x)) != Some(&true) {
        u8::MAX
    } else {
        neighbour_coords(y, x)
            .into_iter()
            .filter(|&(y, x)| *grid.get((y, x)).unwrap_or(&false))
            .count() as u8
    }
}

fn calculate(grid: Array2<bool>) -> (usize, usize) {
    let mut neighbour_counts =
        Array2::<u8>::from_shape_fn(grid.dim(), |(y, x)| neighbours(&grid, y, x));

    let mut changed_q = neighbour_counts
        .indexed_iter()
        .filter(|&(_, elem)| *elem < 4)
        .map(|(c, _)| c)
        .collect::<Vec<_>>();

    let p1 = changed_q.len();
    let mut p2 = 0;

    while let Some((y, x)) = changed_q.pop() {
        for (ny, nx) in neighbour_coords(y, x) {
            if let Some(neighbour) = neighbour_counts.get_mut((ny, nx)) {
                *neighbour = neighbour.saturating_sub(1);
                if *neighbour == 3 {
                    changed_q.push((ny, nx));
                }
            }
        }
        p2 += 1;
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
