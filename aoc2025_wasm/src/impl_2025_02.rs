use anyhow::{Context, Result};
use std::collections::HashSet;

fn string_length_of(n: u64) -> u32 {
    if n == 0 {
        return 1;
    }
    n.ilog10() + 1
}

fn repeat(n: u64, times: u32) -> u64 {
    let len_n = string_length_of(n);
    let mut result = n;

    for _ in 0..(times - 1) {
        result = result * 10_u64.pow(len_n) + n;
    }

    result
}

fn calculate(inp: &str) -> Result<(u64, u64)> {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut invalid_p1 = HashSet::<u64>::default();
    let mut invalid_p2 = HashSet::<u64>::default();

    for elem in inp.split(",") {
        let (start, end) = elem.split_once("-").context("invalid format")?;

        let start: u64 = start.trim().parse()?;
        let start_slen = string_length_of(start);
        let end: u64 = end.trim().parse()?;
        let end_slen = string_length_of(end);

        for postfix_len in 1..=end_slen / 2 {
            let times_lower_bound = start_slen / postfix_len;
            let times_upper_bound = end_slen / postfix_len;

            for postfix in 10_u64.pow(postfix_len - 1)..10_u64.pow(postfix_len) {
                for times in times_lower_bound.max(2)..=times_upper_bound {
                    let n = repeat(postfix, times);

                    if n >= start && n <= end {
                        invalid_p2.insert(n);
                        if times == 2 {
                            invalid_p1.insert(n);
                        }
                    }
                }
            }
        }

        p1 += invalid_p1.iter().sum::<u64>();
        p2 += invalid_p2.iter().sum::<u64>();

        invalid_p1.clear();
        invalid_p2.clear();
    }

    Ok((p1, p2))
}

pub fn run_2025_02(inp: &str) -> Result<String> {
    let (p1, p2) = calculate(inp)?;
    Ok(format!("{p1}\n{p2}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = include_str!("../inputs/examples/2025_02");
    const REAL_DATA: &str = include_str!("../inputs/real/2025_02");

    #[test]
    fn test_example() {
        assert_eq!(calculate(&EXAMPLE_DATA).unwrap(), (1227775554, 4174379265));
    }

    #[test]
    fn test_real() {
        assert_eq!(calculate(&REAL_DATA).unwrap(), (29818212493, 37432260594));
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat(99, 1), 99);
        assert_eq!(repeat(100, 1), 100);
        assert_eq!(repeat(101, 1), 101);

        assert_eq!(repeat(99, 2), 9999);
        assert_eq!(repeat(100, 2), 100100);
        assert_eq!(repeat(101, 2), 101101);

        assert_eq!(repeat(99, 3), 999999);
        assert_eq!(repeat(100, 3), 100100100);
        assert_eq!(repeat(101, 3), 101101101);
    }

    #[test]
    fn test_string_length() {
        assert_eq!(string_length_of(0), 1);
        assert_eq!(string_length_of(9), 1);
        assert_eq!(string_length_of(10), 2);
        assert_eq!(string_length_of(99), 2);
        assert_eq!(string_length_of(100), 3);
    }
}
