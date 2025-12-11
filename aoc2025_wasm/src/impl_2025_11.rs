use anyhow::{Context, Result};
use std::collections::HashMap;

fn connections_from_line(inp: &str) -> Result<Vec<(&str, &str)>> {
    let (here, there) = inp.split_once(": ").context("bad format")?;

    Ok(there
        .split(" ")
        .flat_map(|x| [(here, x)])
        .collect::<Vec<_>>())
}

fn pathfind(connections: &[(&str, &str)], start: &str, end: &str) -> u64 {
    let mut map = HashMap::<&str, u64>::default();
    map.insert(start, 1);

    let mut ans = 0;

    for _ in 0..connections.len() - 1 {
        let mut new = HashMap::<&str, u64>::default();

        connections.iter().for_each(|(s, e)| {
            if let Some(oldval) = map.get(s) {
                *new.entry(e).or_insert(0) += oldval;
            }
        });

        ans += *new.get(end).unwrap_or(&0);

        std::mem::swap(&mut new, &mut map);
    }

    ans
}

fn parse(inp: &str) -> Result<Vec<(&str, &str)>> {
    let connections = inp
        .trim()
        .lines()
        .map(connections_from_line)
        .collect::<Result<Vec<_>>>()?;

    Ok(connections.into_iter().flatten().collect::<Vec<_>>())
}

fn calculate_p1(connections: &[(&str, &str)]) -> u64 {
    pathfind(connections, "you", "out")
}

fn calculate_p2(connections: &[(&str, &str)]) -> u64 {
    let fft_to_dac = pathfind(connections, "fft", "dac");

    if fft_to_dac != 0 {
        pathfind(connections, "svr", "fft") * fft_to_dac * pathfind(connections, "dac", "out")
    } else {
        pathfind(connections, "svr", "dac")
            * pathfind(connections, "dac", "fft")
            * pathfind(connections, "fft", "out")
    }
}

pub fn run_2025_11(inp: &str) -> Result<String> {
    let connections = parse(inp)?;
    let p1 = calculate_p1(&connections);
    let p2 = calculate_p2(&connections);
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_11");
    const EXAMPLE_DATA_P2: &str = include_str!("../inputs/examples/2025_11_2");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_11");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate_p1(&parse(EXAMPLE_DATA).unwrap()), 5);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate_p2(&parse(EXAMPLE_DATA_P2).unwrap()), 2);
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(calculate_p1(&parse(REAL_DATA).unwrap()), 428);
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(calculate_p2(&parse(REAL_DATA).unwrap()), 331468292364745);
    }
}
