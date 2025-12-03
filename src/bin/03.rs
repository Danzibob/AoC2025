advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;
    input.lines().for_each(|line| {
        let bank = line.as_bytes().iter().map(|x| x-0x30).collect::<Vec<u8>>();
        // We need to find the highest digit that isn't the last digit for the tens position
        // and record its index.
        let mut tens_digit = 0;
        let mut max_index = 0;
        for (i, &digit) in bank[..bank.len()-1].iter().enumerate() {
            if digit > tens_digit {
                tens_digit = digit;
                max_index = i;
            }
        }
        // Now we find the maximum digit after the tens position for the units position.
        let mut units_digit = 0;
        for &digit in &bank[max_index+1..] {
            if digit > units_digit {
                units_digit = digit;
            }
        }
        let result = (tens_digit as u64) * 10 + (units_digit as u64);
        total += result;
    });
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;
    input.lines().for_each(|line| {
        let bank = line.as_bytes().iter().map(|x| x-0x30).collect::<Vec<u8>>();
        // This time let's do proper pointer chasing
        let mut joltage:u64 = 0;
        let mut left_ptr:usize = 0; // Updates with the new max digit position

        for position in (0..12).rev() {
            let range = &bank[left_ptr..(bank.len()-position)];
            let mut max_digit = 0;
            let mut max_index = 0;
            for (i, &digit) in range.iter().enumerate() {
                if digit > max_digit {
                    max_digit = digit;
                    max_index = i;
                }
            }
            let multiplier = 10u64.pow(position as u32);
            joltage += multiplier * max_digit as u64;
            left_ptr += max_index + 1; // Move past the found digit
        }
        total += joltage;
    });
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
