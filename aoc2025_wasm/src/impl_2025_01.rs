use anyhow::{Result, anyhow, bail};
use num::traits::Euclid;

fn calculate(raw_inp: &str) -> Result<(i32, i32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut rot = 50;

    for line in raw_inp.lines() {
        let (dir, num) = line
            .split_at_checked(1)
            .ok_or_else(|| anyhow!("invalid line format"))?;

        let num = num.parse::<i32>()?;

        let (full_turns, num) = num.div_rem_euclid(&100);

        p2 += full_turns;

        match dir {
            "L" => {
                rot += num;
                if rot >= 100 {
                    p2 += 1;
                }
            }
            "R" => {
                if rot - num <= 0 && rot > 0 {
                    p2 += 1;
                }
                rot -= num;
            }
            _ => bail!("invalid line format"),
        }

        rot = rot.rem_euclid(100);

        if rot == 0 {
            p1 += 1;
        }
    }

    Ok((p1, p2))
}

pub fn run_2025_01(inp: &str) -> Result<String> {
    let (p1, p2) = calculate(inp)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_01");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_01");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA).unwrap(), (3, 6));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA).unwrap(), (1118, 6289));
    }
}
