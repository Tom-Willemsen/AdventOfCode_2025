use anyhow::{Context, Result};

fn calculate_one<const DIGITS: usize>(line: &[u64]) -> Result<u64> {
    let mut ans = 0;
    let mut current_idx = 0;

    for d in 0..DIGITS {
        let next_digit = line
            .get(current_idx..=line.len() - (DIGITS - d))
            .context("bad slice indices (bad input?)")?
            .iter()
            .max()
            .context("not enough digits in line")?;

        current_idx += line
            .get(current_idx..)
            .context("bad current index")?
            .iter()
            .position(|e| e == next_digit)
            .context("invalid index of next digit")?
            + 1;

        ans *= 10;
        ans += next_digit;
    }

    Ok(ans)
}

fn calculate(inp: &str) -> Result<(u64, u64)> {
    inp.lines()
        .filter_map(|line| {
            let bytes = line
                .bytes()
                .map(|c| c.checked_sub(b'0').map(|c| c as u64))
                .collect::<Option<Vec<_>>>()?;

            Some((
                calculate_one::<2>(&bytes).ok()?,
                calculate_one::<12>(&bytes).ok()?,
            ))
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
        .context("no valid lines?")
}

pub fn run_2025_03(inp: &str) -> Result<String> {
    let (p1, p2) = calculate(inp)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_03");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_03");

    #[test]
    fn test_example_p1() {
        assert_eq!(calculate(EXAMPLE_DATA).unwrap().0, 357);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(calculate(EXAMPLE_DATA).unwrap().1, 3121910778619);
    }

    #[test]
    fn test_real_p1() {
        assert_eq!(calculate(REAL_DATA).unwrap().0, 17107);
    }

    #[test]
    fn test_real_p2() {
        assert_eq!(calculate(REAL_DATA).unwrap().1, 169349762274117);
    }
}
