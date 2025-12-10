use anyhow::{Context, Result, bail};
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

#[derive(Debug)]
struct InputLine {
    indicators: Vec<bool>,
    #[allow(unused)]
    wiring: Vec<Vec<usize>>,
    button_xor_masks: Vec<u64>,
    #[allow(unused)]
    joltages: Vec<i32>,
}

impl FromStr for InputLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, anyhow::Error> {
        let groups = s.split(" ").collect::<Vec<_>>();

        let indicators = groups
            .first()
            .context("Not enough groups on line")?
            .trim_matches(|c| c == '[' || c == ']')
            .bytes()
            .map(|c| c == b'#')
            .collect::<Vec<_>>();

        let wiring: Vec<Vec<usize>> = groups
            .get(1..groups.len() - 1)
            .context("not enough groups")?
            .iter()
            .map(|g| g.trim_matches(|c| c == '(' || c == ')'))
            .map(|g| {
                g.split(",")
                    .map(|n| n.parse::<usize>().context("int parse failed"))
                    .collect::<Result<Vec<usize>, anyhow::Error>>()
            })
            .collect::<Result<_, _>>()?;

        let joltages = groups
            .last()
            .context("not enough groups on line")?
            .trim_matches(|c| c == '{' || c == '}')
            .split(",")
            .map(|n| n.parse::<i32>())
            .collect::<Result<_, _>>()?;

        let button_xor_masks = wiring
            .iter()
            .map(|w| button_to_u64_xor_mask(w))
            .collect::<Vec<_>>();

        Ok(InputLine {
            indicators,
            wiring,
            button_xor_masks,
            joltages,
        })
    }
}

fn indicators_to_u64(indicators: &[bool]) -> u64 {
    let mut result = 0_u64;
    for ind in indicators.iter().rev() {
        result <<= 1;
        result += if *ind { 1 } else { 0 }
    }
    result
}

fn button_to_u64_xor_mask(wiring: &[usize]) -> u64 {
    let mut result = 0;
    for itm in wiring.iter() {
        result |= 1 << itm
    }
    result
}

fn calculate_p1_line(line: &InputLine) -> Result<u64> {
    let target_state = indicators_to_u64(&line.indicators);

    // (state, num_pushes)
    let mut q = VecDeque::new();
    let mut s: HashSet<(u64, u64)> = HashSet::default();

    q.push_back((0, 0));

    while let Some((state, num_pushes)) = q.pop_front() {
        if state == target_state {
            return Ok(num_pushes);
        }

        for mask in line.button_xor_masks.iter() {
            let new_state = state ^ *mask;
            if s.insert((new_state, num_pushes + 1)) {
                q.push_back((new_state, num_pushes + 1));
            }
        }
    }

    bail!("no p1 solution");
}

fn calculate(inp: &str) -> Result<(u64, &str)> {
    let lines = inp
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<InputLine>>>()?;

    let p1 = lines.iter().map(calculate_p1_line).sum::<Result<u64>>()?;

    Ok((p1, "p2 not implemented yet"))
}

pub fn run_2025_10(inp: &str) -> Result<String> {
    let (p1, p2) = calculate(inp)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_10");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_10");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate(EXAMPLE_DATA).unwrap().0, 7);
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(calculate(REAL_DATA).unwrap().0, 422);
    }
}
