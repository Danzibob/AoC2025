advent_of_code::solution!(4);

use grid::Grid;

pub fn part_one(input: &str) -> Option<u64> {
    // Before we construct the grid we need to determine its size
    let first_line: &str = input.lines().next()?;
    let line_length = first_line.len();
    // Now we turn the whole input into a single vector for Grid
    let numbers: Vec<u32> = input
        .lines()
        .flat_map(|line| line.chars().map(|c| if c == '@' { 1 } else { 0 }))
        .collect();
    let grid = Grid::from_vec(numbers, line_length);

    let mut count = 0;
    // For every 1 in the grid, check its neighbors
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if let Some(i) = grid.get(row, col)
                && *i == 1
            {
                let mut neighbor_count = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if let Some(value) =
                            grid.get((row as isize + dx) as usize, (col as isize + dy) as usize)
                        {
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

// Get adjacent coordinates
// Wraps usize to avoid underflow
// This is safe because we check bounds when accessing the grid
fn adj(row: usize, col: usize) -> [(usize, usize); 8] {
    [
        (row.wrapping_sub(1), col.wrapping_sub(1)),
        (row.wrapping_sub(1), col),
        (row.wrapping_sub(1), col + 1),
        (row, col.wrapping_sub(1)),
        (row, col + 1),
        (row + 1, col.wrapping_sub(1)),
        (row + 1, col),
        (row + 1, col + 1),
    ]
}

fn count_adj(grid: &Grid<bool>, row: usize, col: usize) -> u32 {
    let mut count = 0;
    for (r, c) in adj(row, col) {
        if let Some(value) = grid.get(r, c) && *value {
            count += 1;
        }
    }
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    // Before we construct the grid we need to determine its size
    let line_length = input.find('\n').unwrap() + 1;
    // Now we turn the whole input into a single vector for Grid
    let numbers: Vec<bool> = input.as_bytes().iter().map(|c| *c == b'@').collect();
    let mut grid = Grid::from_vec(numbers, line_length);

    let mut changed = true;
    let mut removed = 0;
    while changed {
        changed = false;
        for row in 0..grid.rows() {
            for col in 0..grid.cols() {
                if *grid.get(row, col)? {
                    let count = count_adj(&grid, row, col);
                    if count < 4 {
                        *grid.get_mut(row, col)? = false;
                        changed = true;
                        removed += 1;
                    }
                }
            }
        }
    }
    Some(removed)
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
        assert_eq!(result, Some(43));
    }
}
