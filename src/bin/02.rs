advent_of_code::solution!(2);

struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        Range { start, end }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    // To find repeated digit patterns, we can check for multiples of
    // numbers like 11, 101, 1001, etc. where the other factor is less digits.
    // e.g. 11 * 9 = 99, 101 * 9 = 909, 1001 * 9 = 9009
    // However if the other factor is longer this doesn't work:
    // e.g. 11 * 99 = 1089, 101 * 999 = 100899
    // it also doesn't work for factors which are too short b/c of the leading zeros rule:
    // e.g. 101 * 2 = 202, 1001 * 24 = 24024
    // Therefore for each pattern (11, 101, etc.) we need to find the valid range of factors
    // 11: 1-9       | 11  *1  =11    ... 11*9=99
    // 101: 10-99    | 101 *10 = 1010 ... 101*99=9999
    // 1001: 100-999  | 1001*100=100100 ... 1001*999=999999
    // etc.
    let mut sum: u64 = 0;
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(Range::from_str)
        .for_each(|r| {
            let upper_power = (r.end as f64).log10().floor() as u32;
            for power in 1..=upper_power {
                let base = 10u32.pow(power); // e.g. 10, 100, 1000, ...
                let pattern = (base + 1) as f64; // e.g. 11, 101, 1001, ...
                let factor_range = Range {
                    start: (base / 10) as u64,
                    end: (base - 1) as u64,
                }; // e.g. 1-9, 10-99, 100-999, ...

                // Now we calculate the range of valid factors for this pattern
                let min_factor = (r.start as f64 / pattern).ceil() as u64;
                let max_factor = (r.end as f64 / pattern).floor() as u64;

                // Then intersect with the valid factor range
                let valid_start = min_factor.max(factor_range.start);
                let valid_end = max_factor.min(factor_range.end);

                // And now we can count the valid factors using the 
                // arithmetic series formula: n/2 * (first + last)
                if valid_start <= valid_end {
                    let count = valid_end - valid_start + 1;
                    sum += count * (valid_start + valid_end) / 2 * (pattern as u64);
                }
            }
        });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Now we have to deal with different counts of repeated digits
    // e.g. 121212 is 12 repeated 3 times so is invalid
    // We can use the same trick as part one but now we have to consider
    // patterns like 10101 (for 3 repeats) and 1111 (for 4 repeats)
    // 10101 has a factor range of 10-99 and 1111 has a factor range of 1-9
    // We can generalize this to:
    // factor range 1-9: 11 = (10^1 + 10^0), 1111... = sum(10^i for i in 0..4)
    // factor range 10-99: 101 = (10^2 + 10^0), 10101... = sum(10^i for i in (0..5).step_by(2))
    // etc.
    // We can then use the same logic as part one to find the valid factors
    // HOWEVER we now need to worry about duplicates as 2222 is 101 * 22 and 1111 * 2
    let mut ids: std::collections::HashSet<u64> = std::collections::HashSet::new();
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(Range::from_str)
        .for_each(|r| {
            let max_digits = (r.end as f64).log10().floor() as u32;
            for factor_digits in 1..=max_digits {
                let factor_range = Range {
                    start: 10u64.pow(factor_digits - 1),
                    end: 10u64.pow(factor_digits) - 1,
                }; // e.g. 1-9, 10-99, 100-999, ...
                for repetitions in 2..=(max_digits/factor_digits + 1) {
                    // Build the pattern
                    let mut pattern: u64 = 0;
                    for rep in 0..repetitions {
                        pattern += 10u64.pow(rep * factor_digits);
                    } // 10^0 + 10^n + 10^(2n) + ... = 111, 10101010, etc.

                    // Now we calculate the range of valid factors for this pattern
                    let min_factor = (r.start as f64 / pattern as f64).ceil() as u64;
                    let max_factor = (r.end as f64 / pattern as f64).floor() as u64;

                    // Then intersect with the valid factor range
                    let valid_start = min_factor.max(factor_range.start);
                    let valid_end = max_factor.min(factor_range.end);

                    // And now we can collect the invalid IDs
                    for v in valid_start..=valid_end {
                        ids.insert(v * pattern);
                    }
                }
            }});
    // Finally sum the unique IDs
    Some(ids.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
