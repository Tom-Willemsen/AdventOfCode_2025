use anyhow::{Result, anyhow};

fn calculate_one<const DIGITS: usize>(line: &[u64]) -> Result<u64> {
    let mut ans = 0;
    let mut current_idx = 0;

    for d in 0..DIGITS {
        let next_digit = line
            .get(current_idx..=line.len() - (DIGITS - d))
            .ok_or_else(|| anyhow!("bad slice indices (bad input?)"))?
            .iter()
            .max()
            .ok_or_else(|| anyhow!("not enough digits in line"))?;

        current_idx += line
            .get(current_idx..)
            .ok_or_else(|| anyhow!("bad current index"))?
            .iter()
            .position(|e| e == next_digit)
            .ok_or_else(|| anyhow!("invalid index of next digit"))?
            + 1;

        ans *= 10;
        ans += next_digit;
    }

    Ok(ans)
}

fn calculate<const DIGITS: usize>(inp: &str) -> Result<u64> {
    inp.lines()
        .map(|line| {
            let bytes = line
                .bytes()
                .map(|c| {
                    c.checked_sub(b'0')
                        .ok_or_else(|| anyhow!("invalid character"))
                        .map(|c| c as u64)
                })
                .collect::<Result<Vec<_>>>()?;

            calculate_one::<DIGITS>(&bytes)
        })
        .sum::<Result<u64>>()
}

pub fn run_2025_03(inp: &str) -> Result<String> {
    let p1 = calculate::<2>(inp)?;
    let p2 = calculate::<12>(inp)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_03");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_03");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate::<2>(EXAMPLE_DATA).unwrap(), 357);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate::<12>(EXAMPLE_DATA).unwrap(), 3121910778619);
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(calculate::<2>(REAL_DATA).unwrap(), 17107);
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(calculate::<12>(REAL_DATA).unwrap(), 169349762274117);
    }
}
