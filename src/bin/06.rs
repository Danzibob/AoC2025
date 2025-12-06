advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().collect();

    // Split the last line off as the operators
    let (val_lines, op_lines) = lines.split_at(lines.len() - 1);

    // Create an iterator over the value lines, parsing each number as u64
    let mut vals: Vec<_> = val_lines.iter().map(|line| {
        line.split_whitespace()
            .map(|num_str| num_str.parse::<u64>().unwrap())
    }).collect();

    // Now create a similar iterator for the operators
    let ops = op_lines.first().unwrap().split_whitespace().map(|op_str| {
        op_str.chars().next().unwrap()
    });

    // Now we iterate over the values and operators together
    Some(ops.fold(0u64, |total, op| {
            let column = vals.iter_mut().map(|row| {
                row.next().unwrap()
        });
            let result: u64 = match op {
                '+' => column.sum(),
                '*' => column.product(),
                _ => panic!("Unknown operator"),
            };
            total + result
        })
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    // Yeesh this one looks like a pain
    // We can find the split points by looking at the operators row: 
    // the empty column always preceeds the operator
    let lines = input.lines().collect::<Vec<&str>>();
    
    // Let's identify all the split points first
    let op_line = lines.last().unwrap();
    let mut split_points: Vec<usize> = op_line.char_indices()
        .filter(|&(_, c)| c != ' ')
        .map(|(i, _)| i)
        .collect();
    let ops = op_line.split_whitespace().map(|op_str| {
        op_str.chars().next().unwrap()
    });
    // We also need to append the end of the line as a split point so we can use window(2)
    split_points.push(op_line.len() + 1); // +1 to simulate the blank column at the end

    // Check the max width of the input - this is the number of values in each problem
    let rows = lines.len() - 1;

    // Now we iterate over the split points and solve the problems individually
    // (Note: this seems like a good candidate for rayon!)
    let tot: u64 = split_points.windows(2).zip(ops).map(|(window, op)| {
        let start = window[0];
        let end = window[1]-1; // Exclude the blank column
        let cols = end - start;

        let vals = (0..cols).map(|c| {
            let mut val = 0;
            for row in lines.iter().take(rows) {
                match row.chars().nth(start + c) {
                    Some(ch) if ch.is_ascii_digit() => {
                        let d = ch.to_digit(10).unwrap() as u64;
                        val = val * 10 + d;
                    },
                    _ => {}

                }
                
            }
            val
        });

        //print!("{:?} with op {}\n", vals.clone().collect::<Vec<u64>>(), op);

        match op {
            '+' => {vals.sum::<u64>()},
            '*' => {vals.product::<u64>()},
            _ => panic!("Unknown operator"),
        }
    }).sum();

    Some(tot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
