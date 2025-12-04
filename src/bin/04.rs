advent_of_code::solution!(4);

use grid::Grid;

pub fn part_one(input: &str) -> Option<u64> {
    // Before we construct the grid we need to determine its size
    let first_line: &str = input.lines().next()?;
    let line_length = first_line.len();
    // Now we turn the whole input into a single vector for Grid
    let numbers: Vec<u32> = input
        .lines()
        .flat_map(|line| line.chars().map(|c|{
            if c == '@' {1} else {0} 
        }))
        .collect();
    let grid = Grid::from_vec(numbers, line_length);

    let mut count = 0;
    // For every 1 in the grid, check its neighbors
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if let Some(i) = grid.get(row, col) && *i == 1 {
                let mut neighbor_count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if let Some(value) = grid.get((row as isize + dx) as usize, (col as isize + dy) as usize) {
                            neighbor_count += *value;
                        }
                    }
                }
                if neighbor_count <= 4 {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
