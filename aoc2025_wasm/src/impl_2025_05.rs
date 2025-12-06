use ahash::AHashSet;
use anyhow::{Context, Result};
use itertools::Itertools;

type Input = Result<(Vec<(u64, u64)>, Vec<u64>)>;

fn parse(raw_input: &str) -> Input {
    let (ranges, ingredients) = raw_input.split_once("\n\n").context("invalid format")?;

    let ranges: Vec<(u64, u64)> = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").context("invalid range format")?;
            Ok((start.parse::<u64>()?, end.parse::<u64>()? + 1))
        })
        .collect::<Result<Vec<_>>>()?;

    let ingredients: Vec<u64> = ingredients
        .lines()
        .map(|line| line.parse::<u64>().context("parsing ingredient failed"))
        .collect::<Result<Vec<_>>>()?;

    Ok((ranges, ingredients))
}

fn calculate_p1(ranges: &[(u64, u64)], ingredients: &[u64]) -> usize {
    ingredients
        .iter()
        .filter(|&&i| ranges.iter().any(|&r| r.0 <= i && r.1 > i))
        .count()
}

fn split_at_boundaries(range: (u64, u64), boundaries: &[u64]) -> Vec<(u64, u64)> {
    let mut output = vec![];
    output.push(range.0);
    for &b in boundaries {
        if b > range.0 && b < range.1 {
            output.push(b);
        }
    }
    output.push(range.1);

    output.into_iter().tuple_windows().collect::<Vec<_>>()
}

fn calculate_p2(ranges: &[(u64, u64)]) -> u64 {
    let boundaries = ranges
        .iter()
        .flat_map(|r| [r.0, r.1])
        .sorted()
        .dedup()
        .collect::<Vec<_>>();

    let mut fresh = AHashSet::default();
    let mut p2 = 0;

    for &range in ranges {
        for subrange in split_at_boundaries(range, &boundaries) {
            if fresh.insert(subrange) {
                p2 += subrange.1 - subrange.0;
            }
        }
    }
    p2
}

pub fn run_2025_05(inp: &str) -> Result<String> {
    let (ranges, ingredients) = parse(inp)?;
    let p1 = calculate_p1(&ranges, &ingredients);
    let p2 = calculate_p2(&ranges);
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_05");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_05");

    #[test]
    fn test_example_p1() {
        let (ranges, ingredients) = parse(EXAMPLE_DATA).unwrap();
        assert_eq!(calculate_p1(&ranges, &ingredients), 3);
    }

    #[test]
    fn test_example_p2() {
        let (ranges, _) = parse(EXAMPLE_DATA).unwrap();
        assert_eq!(calculate_p2(&ranges), 14);
    }

    #[test]
    fn test_real_p1() {
        let (ranges, ingredients) = parse(REAL_DATA).unwrap();
        assert_eq!(calculate_p1(&ranges, &ingredients), 694);
    }

    #[test]
    fn test_real_p2() {
        let (ranges, _) = parse(REAL_DATA).unwrap();
        assert_eq!(calculate_p2(&ranges), 352716206375547);
    }
}
