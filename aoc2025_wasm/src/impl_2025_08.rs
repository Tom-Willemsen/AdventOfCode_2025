use crate::union_find::UfNode;
use std::collections::BinaryHeap;
use std::str::FromStr;
use std::{cmp::Reverse, hash::Hash};

use anyhow::{Context, Result, ensure};
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn dist_squared(&self, other: &Coord) -> i64 {
        let xd = self.x - other.x;
        let yd = self.y - other.y;
        let zd = self.z - other.z;
        xd * xd + yd * yd + zd * zd
    }
}

impl FromStr for Coord {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(",")
            .collect_tuple()
            .context("incorrect number of elements in line")?;

        Ok(Coord {
            x: x.parse()?,
            y: y.parse()?,
            z: z.parse()?,
        })
    }
}

fn calculate<const P1_CONNECTIONS: usize>(inp: &str) -> Result<(usize, i64)> {
    let coords = inp
        .lines()
        .map(|line| line.parse::<Coord>())
        .zip(0_usize..)
        .map(|(c, i)| c.map(|c| (c, i)))
        .collect::<Result<Vec<_>>>()?;

    let mut distances = coords
        .iter()
        .tuple_combinations()
        .map(|((b1, id1), (b2, id2))| (Reverse(b1.dist_squared(b2)), id1, id2, b1, b2))
        .collect::<BinaryHeap<_>>();

    let mut circuits: UfNode = UfNode::new((0..coords.len()).collect::<Vec<_>>());
    let mut connections = 0;

    let mut p1 = 0;
    let mut p2 = 0;

    while let Some((_, id1, id2, b1, b2)) = distances.pop() {
        circuits.union_sets(*id1, *id2);

        connections += 1;

        if connections == P1_CONNECTIONS {
            let mut circuit_sizes = circuits.sizes();
            let partition_point = circuit_sizes.len();
            let (_, _, top3) = circuit_sizes.select_nth_unstable(partition_point.saturating_sub(4));
            p1 = top3.iter().product();
        }

        if circuits.num_sets() == 1 {
            ensure!(p1 != 0, "solved part 2 before part 1?");
            p2 = b1.x * b2.x;
            break;
        }
    }

    Ok((p1, p2))
}

pub fn run_2025_08(inp: &str) -> Result<String> {
    let (p1, p2) = calculate::<1000>(inp)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_08");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_08");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate::<10>(EXAMPLE_DATA).unwrap().0, 40);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate::<10>(EXAMPLE_DATA).unwrap().1, 25272);
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(calculate::<1000>(REAL_DATA).unwrap().0, 96672);
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(calculate::<1000>(REAL_DATA).unwrap().1, 22517595);
    }
}
