use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::{cell::RefCell, collections::HashSet, str::FromStr};

use anyhow::{Context, Result, ensure};
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

fn calculate_p1(circuits: &[RefCell<HashSet<&Coord>>]) -> usize {
    circuits
        .iter()
        .map(|c| c.borrow().len())
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

fn calculate<const P1_CONNECTIONS: usize>(inp: &str) -> Result<(usize, i64)> {
    let coords = inp
        .lines()
        .map(|line| line.parse::<Coord>())
        .collect::<Result<Vec<_>>>()?;

    let mut distances = coords
        .iter()
        .tuple_combinations()
        .map(|(b1, b2)| (Reverse(b1.dist_squared(b2)), b1, b2))
        .collect::<BinaryHeap<_>>();

    let mut circuits: Vec<RefCell<HashSet<&Coord>>> = vec![];
    let mut connections = 0;

    let mut p1 = 0;
    let mut p2 = 0;

    while let Some((_, b1, b2)) = distances.pop() {
        let mut added = false;
        for c in circuits.iter() {
            let mut c = c.borrow_mut();
            if c.contains(b1) || c.contains(b2) {
                c.insert(b1);
                c.insert(b2);
                for c2 in circuits.iter() {
                    if let Ok(mut c2) = c2.try_borrow_mut()
                        && (c2.contains(b1) || c2.contains(b2))
                    {
                        c.extend(c2.iter());
                        c2.clear();
                    }
                }
                added = true;
                break;
            }
        }

        if !added {
            let new_circuit = HashSet::from([b1, b2]);
            circuits.push(RefCell::new(new_circuit));
        }

        circuits.retain(|c| !c.borrow().is_empty());

        connections += 1;

        if connections == P1_CONNECTIONS {
            p1 = calculate_p1(&circuits);
        }

        let max_circuit_length = circuits
            .iter()
            .map(|c| c.borrow().len())
            .max()
            .context("no circuits")?;

        if max_circuit_length == coords.len() {
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
