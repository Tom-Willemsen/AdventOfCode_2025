use anyhow::{Context, Result};
use itertools::Itertools;
use ndarray::{Array2, s};
use std::{cmp::Reverse, collections::VecDeque};

fn parse(inp: &str) -> Result<Vec<(u64, u64)>> {
    inp.trim()
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").context("bad format")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect()
}

fn area(p1: (u64, u64), p2: (u64, u64)) -> u64 {
    (p2.0.abs_diff(p1.0) + 1) * (p2.1.abs_diff(p1.1) + 1)
}

fn valid_part2((xs, ys): (usize, usize), (xe, ye): (usize, usize), grid: &Array2<State>) -> bool {
    // 'quick' pre check that all 4 corners are filled in - saves substantial time.
    grid[(xs, ys)] != State::Empty
        && grid[(xs, ye)] != State::Empty
        && grid[(xe, ys)] != State::Empty
        && grid[(xe, ye)] != State::Empty
        && !grid
            .slice(s![xs..=xe, ys..=ye])
            .into_iter()
            .contains(&State::Empty)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Undefined,
    Empty,
    Occupied,
}

type Data = [((u64, u64), (usize, usize))];

fn fill(data: &Data, grid: &mut Array2<State>) -> Option<()> {
    let mut prev_pt = data.last()?.1;

    for (_, indices) in data.iter() {
        let sx = prev_pt.0;
        let sy = prev_pt.1;
        let ex = indices.0;
        let ey = indices.1;

        if sy == ey {
            for x in sx.min(ex) + 1..sx.max(ex) {
                grid[(x, sy)] = State::Occupied;
            }
        } else {
            for y in sy.min(ey) + 1..sy.max(ey) {
                grid[(sx, y)] = State::Occupied;
            }
        }
        grid[(sx, sy)] = State::Occupied;
        grid[(ex, ey)] = State::Occupied;

        prev_pt = *indices;
    }
    Some(())
}

fn floodfill(grid: &mut Array2<State>) {
    let mut q = VecDeque::with_capacity(4096);
    q.push_back((0_usize, 0_usize));

    while let Some((cx, cy)) = q.pop_front() {
        if grid[(cx, cy)] != State::Undefined {
            continue;
        }
        grid[(cx, cy)] = State::Empty;

        for (x, y) in [
            (cx + 1, cy),
            (cx.wrapping_sub(1), cy),
            (cx, cy + 1),
            (cx, cy.wrapping_sub(1)),
        ] {
            if grid.get((x, y)) == Some(&State::Undefined) {
                q.push_back((x, y));
            }
        }
    }
}

fn calculate(inp: &str) -> Result<(u64, u64)> {
    let data = parse(inp)?;

    let xs = data
        .iter()
        .flat_map(|&(x, _)| [x - 1, x, x + 1])
        .sorted_unstable()
        .dedup()
        .collect::<Vec<_>>();

    let ys = data
        .iter()
        .flat_map(|&(_, y)| [y - 1, y, y + 1])
        .sorted_unstable()
        .dedup()
        .collect::<Vec<_>>();

    let data = data
        .into_iter()
        .map(|(x, y)| {
            Ok((
                (x, y),
                (
                    xs.binary_search(&x).ok().context("binary search fail")?,
                    ys.binary_search(&y).ok().context("binary search fail")?,
                ),
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    // What kind of psychopath uses (x, y) indexing !??
    let mut grid = Array2::from_elem((xs.len(), ys.len()), State::Undefined);

    fill(&data, &mut grid);
    floodfill(&mut grid);

    let mut combinations_by_area = data
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| {
            (
                Reverse(area(p1.0, p2.0)),
                (p1.1.0.min(p2.1.0), p1.1.1.min(p2.1.1)),
                (p1.1.0.max(p2.1.0), p1.1.1.max(p2.1.1)),
            )
        })
        .collect::<Vec<_>>();

    combinations_by_area.sort_unstable_by_key(|e| e.0);

    let p1 = combinations_by_area
        .first()
        .map(|&(area, _, _)| area)
        .context("not enough combinations?")?;

    let p2 = combinations_by_area
        .iter()
        .find(|(_, pt1, pt2)| valid_part2(*pt1, *pt2, &grid))
        .map(|&(area, _, _)| area)
        .context("no p2 solution")?;

    Ok((p1.0, p2.0))
}

pub fn run_2025_09(inp: &str) -> Result<String> {
    let (p1, p2) = calculate(inp)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_09");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_09");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate(EXAMPLE_DATA).unwrap().0, 50);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate(EXAMPLE_DATA).unwrap().1, 24);
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(calculate(REAL_DATA).unwrap().0, 4741451444);
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(calculate(REAL_DATA).unwrap().1, 1562459680);
    }

    #[test]
    fn test_tricky_input() {
        // Tricky as it has edges 'within' the shape, but no empty space
        let tricky = "
1,1
4,1
4,4
5,4
5,1
10,1
10,10
1,10
";
        assert_eq!(calculate(tricky).unwrap(), (100, 100));
    }

    #[test]
    fn test_1_wide_l_input() {
        // L-shaped input, 1 wide.
        let tricky = "
1,1
1,10
1,1
15,1
";
        assert_eq!(calculate(tricky).unwrap(), (150, 15));
    }

    #[test]
    fn test_2_wide_l_input() {
        // L-shaped input, 2 wide.
        let tricky = "
1,1
1,10
2,10
2,2
10,2
10,1
";
        assert_eq!(calculate(tricky).unwrap(), (100, 20));
    }
}
