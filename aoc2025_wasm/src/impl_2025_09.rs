use anyhow::{Context, Result};
use itertools::Itertools;
use rayon::prelude::*;
use std::cmp::Reverse;

fn parse(inp: &str) -> Result<Vec<(u64, u64)>> {
    inp.lines()
        .map(|line| {
            let (a, b) = line.split_once(",").context("bad format")?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect()
}

fn area(p1: (u64, u64), p2: (u64, u64)) -> u64 {
    (p2.0.abs_diff(p1.0) + 1) * (p2.1.abs_diff(p1.1) + 1)
}

fn valid_part2(
    p1: (u64, u64),
    p2: (u64, u64),
    x_cutoffs: &[(u64, u64, u64)],
    y_cutoffs: &[u64],
) -> bool {
    let min_x = p1.0.min(p2.0);
    let max_x = p1.0.max(p2.0);
    let min_y = p1.1.min(p2.1);
    let max_y = p1.1.max(p2.1);

    for &y in y_cutoffs.iter() {
        if y > max_y {
            break;
        }
        if y < min_y {
            continue;
        }
        for cutoff in x_cutoffs.iter() {
            if cutoff.0 > max_x {
                break;
            }
            if cutoff.0 <= min_x || !(cutoff.1..cutoff.2).contains(&y) {
                continue;
            }
            return false;
        }
    }

    true
}

fn calculate(inp: &str) -> Result<(u64, u64)> {
    let data = parse(inp)?;

    let mut x_cutoffs = vec![];

    let mut prev_pt = *data.last().context("not enough data?")?;

    for pt in data.iter() {
        if pt.1 > prev_pt.1 {
            x_cutoffs.push((pt.0, prev_pt.1, pt.1 + 1));
        } else if pt.1 < prev_pt.1 {
            x_cutoffs.push((pt.0, pt.1 + 1, prev_pt.1))
        }

        prev_pt = *pt;
    }

    x_cutoffs.sort_unstable();

    let y_cutoffs = data
        .iter()
        .flat_map(|&(_, y)| [y, y + 1])
        .sorted_unstable()
        .dedup()
        .collect::<Vec<_>>();

    let mut combinations_by_area = data
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| (Reverse(area(*p1, *p2)), p1, p2))
        .collect::<Vec<_>>();

    combinations_by_area.par_sort_unstable();

    let p1 = combinations_by_area
        .first()
        .map(|&(area, _, _)| area.0)
        .context("not enough combinations?")?;

    let p2 = combinations_by_area
        .par_iter()
        .by_exponential_blocks()
        .find_first(|&(_, p1, p2)| valid_part2(**p1, **p2, &x_cutoffs, &y_cutoffs))
        .map(|&(area, _, _)| area.0)
        .context("no p2 solution")?;

    Ok((p1, p2))
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
}
