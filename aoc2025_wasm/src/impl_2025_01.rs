use ahash::AHashMap;
use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::iter::zip;

fn calculate(raw_inp: &str) -> Result<(i32, i32)> {
    let mut left = vec![];
    let mut right = vec![];

    for line in raw_inp.lines() {
        let (l, r) = line.split_once("   ").ok_or(anyhow!("bad format"))?;
        left.push(l.parse()?);
        right.push(r.parse()?);
    }

    left.sort_unstable();
    right.sort_unstable();

    let right_count: AHashMap<i32, i32> = right
        .iter()
        .dedup_with_count()
        .map(|(count, v)| (*v, count as i32))
        .collect();

    let p1 = zip(&left, &right).map(|(l, r)| (l - r).abs()).sum();

    let p2 = left
        .into_iter()
        .map(|l| l * right_count.get(&l).unwrap_or(&0))
        .sum();

    Ok((p1, p2))
}

pub fn run_2025_01(inp: &str) -> Result<String> {
    let (p1, p2) = calculate(inp)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_one_plus_one() {
        assert_eq!(1 + 1, 2);
    }
}
