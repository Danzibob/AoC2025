advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut pos: isize = 50;
    let mut zero_count: u64 = 0;
    input.lines().for_each(|line| {
        let l = line.as_bytes();
        if l[0] == b'L' {
            pos -= line[1..].parse::<isize>().unwrap();
        } else {
            pos += line[1..].parse::<isize>().unwrap();
        }
        if (pos % 100) == 0 {
            zero_count += 1;
        }
    });
    Some(zero_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut pos: isize = 50;
    let mut zero_count: u64 = 0;
    input.lines().for_each(|line| {
        let l = line.as_bytes();
        let was_on_zero = pos == 0;

        if l[0] == b'L' {
            pos -= line[1..].parse::<isize>().unwrap();
        } else {
            pos += line[1..].parse::<isize>().unwrap();
        }

        // Count how many hundreds we've passed
        let delta = pos.abs() / 100;

        if pos > 99 {
            // Moving right we just count the hundreds passed
            zero_count += delta as u64;
        } else if pos < 0 {
            // Moving left we count the hundreds passed plus one if we
            // weren't already on zero
            zero_count += delta as u64 + if was_on_zero { 0 } else { 1 };
        } else if pos == 0 {
            // Landed exactly on zero
            zero_count += 1;
        }

        // Remap position within 0..99
        pos = (pos + 1000) % 100;
    });
    Some(zero_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
