advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    // Split the input into the two sections
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    // Parse the items first as it's easier to search through items than lines
    let mut items: Vec<u64> = input_parts.get(1)?
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    // Sort the items for easier searching
    items.sort_unstable();
    // Create a seperate vector to count without modifying or double counting
    let mut counted = vec![false; items.len()];

    // Now we can binary search through the items list to find the right
    // place to check for each range
    input_parts.get(0)?.lines().for_each(|line| {
        // Lines of format XX-YY for arbitrary integers XX and YY
        let mut parts = line.split('-').into_iter();
        let start: u64 = parts.next().unwrap().parse().unwrap();
        let end: u64 = parts.next().unwrap().parse().unwrap();
        // Find the first item that is >= start
        let start_index = match items.binary_search(&start) {
            Ok(index) => index,
            Err(index) => index,
        };
        // And find the first item that is >= end
        let end_index = match items.binary_search(&end) {
            Ok(index) => index + 1, // Include the item if it matches exactly
            Err(index) => index,
        };
        // Now we can just set the counted flags for all items in this range
        counted[start_index..end_index].iter_mut().for_each(|flag| *flag = true);
    });
    // Finally count all the counted flags that are true
    Some(counted.into_iter().filter(|&flag| flag).count() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    // This time we don't care about the items, just the ranges
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let mut ranges: Vec<(u64, u64)> = input_parts.get(0)?
        .lines()
        .map(|line| {
            let mut parts = line.split('-').into_iter();
            let start: u64 = parts.next().unwrap().parse().unwrap();
            let end: u64 = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();
    // First lets sort them by start value
    ranges.sort_unstable_by_key(|&(start, _)| start);
    // Now let's create a Vec to track counted ranges
    let mut merged: Vec<(u64,u64)> = Vec::with_capacity(ranges.len());
    for range in ranges {
        // If the counted list is empty, just add the range
        if merged.is_empty() {
            merged.push(range);
            continue;
        }
        // Otherwise check if it overlaps with the last range
        let last_range = merged.last_mut().unwrap();
        if range.0 <= last_range.1 {
            // It overlaps, so we need to merge them
            if range.1 > last_range.1 {
                last_range.1 = range.1;
            } // else it's fully contained, do nothing
        } else {
            // No overlap, just add the range
            merged.push(range);
        }
    }

    // Now we can calculate the total covered range
    let total = merged.iter()
        .map(|&(start, end)| end - start + 1)
        .sum();
    Some(total)
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
        assert_eq!(result, Some(14));
    }
}
