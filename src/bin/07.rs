advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let first_line = input.lines().next()?;
    let start_index = first_line.find('S')?;
    let width = first_line.len();
    // Create a bool array with width elements
    let mut rays = vec![false; width];
    let mut split_rays = vec![false; width];
    rays[start_index] = true;

    let mut splits = 0;

    // Now we parse every OTHER line of the input, starting from the third line
    for line in input.lines().skip(2).step_by(2) {
        // copy rays into split_rays
        split_rays.copy_from_slice(&rays);
        // We only care about the characters between the bounds
        line.char_indices()
            .filter(|(_, c)| c == &'^')
            .for_each(|(i, _)|{
                // 
                if rays[i] {
                    split_rays[i-1] |= true;
                    split_rays[i] = false;
                    split_rays[i+1] = true;
                    splits += 1;
                }
            });
        // copy split_rays back into rays
        rays.copy_from_slice(&split_rays);
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let first_line = input.lines().next()?;
    let start_index = first_line.find('S')?;
    let width = first_line.len();
    // This time let's count total rays instead of just presence of a ray
    let mut rays = vec![0u64; width];
    let mut split_rays = vec![0u64; width];
    rays[start_index] = 1;

    // Now we parse every OTHER line of the input, starting from the third line
    for line in input.lines().skip(2).step_by(2) {
        // copy rays into split_rays
        split_rays.copy_from_slice(&rays);
        // We only care about the characters between the bounds
        line.char_indices()
            .filter(|(_, c)| c == &'^')
            .for_each(|(i, _)|{
                let universes = rays[i];
                if universes > 0 {
                    // Distribute the universes - one to the left, one to the right
                    // and none straight ahead as the ray splits
                    split_rays[i-1] += universes;
                    split_rays[i] -= universes;
                    split_rays[i+1] += universes;
                }
            });
        // copy split_rays back into rays
        rays.copy_from_slice(&split_rays);
    }

    Some(rays.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
