advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
struct Point(isize, isize);

impl Point {
    fn transform(&self, basis: (isize, isize)) -> Self {
        Point(self.0 * basis.0, self.1 * basis.1)
    }
}

const NE: (isize, isize) = (-1, 1);
const NW: (isize, isize) = (1, 1);
const SE: (isize, isize) = (-1, -1);
const SW: (isize, isize) = (1, -1);

pub fn part_one(input: &str) -> Option<u64> {
    // Load the tiles from the grid as a sparse matrix
    let tiles: Vec<Point> = input
        .lines()
        .map(|line| {
            let mut items = line.split(',');
            Point(
                items.next().unwrap().parse::<isize>().unwrap(),
                items.next().unwrap().parse::<isize>().unwrap(),
            )
        }).collect();

    // split the tiles into quadrants based on the origin
    // and remove any that are pareto dominated in their quadrant
    let mut quadrant_tiles: Vec<Vec<Point>> = vec![Vec::new(); 4];
    for (q, quadrant) in [NE, NW, SE, SW].iter().enumerate() {
        let q_tiles: Vec<Point> = tiles.iter()
            .map(|tile| {
                // Transform the tile into the ++ quadrant
                tile.transform(*quadrant)
            }).collect();
        
        // println!("Quadrant {}: initial tiles: {:?}", q, q_tiles);
        
        // Find the non-dominated pareto frontier in this quadrant
        quadrant_tiles[q] = q_tiles.iter().copied()
            .filter(|&p| {
                !q_tiles.iter().any(|&q| {
                    (q.0 >= p.0 && q.1 >= p.1) && (q.0 > p.0 || q.1 > p.1)
                })
            }).map(|p| p.transform(*quadrant))// transform back to original quadrant
            .collect();
        
        // println!("Quadrant {}: {:?}", q, quadrant_tiles[q]);
    }

    // Now for each pair of quadrants (NE, SW) and (NW, SE)
    // check pairs to find the maximum area rectangle

    let mut max_area: u64 = 0;

    // NE and SW
    for ne_tile in quadrant_tiles[0].iter() {
        for sw_tile in quadrant_tiles[3].iter() {
            let width = ne_tile.0 - sw_tile.0;
            let height = ne_tile.1 - sw_tile.1;
            let area = ((width+1).abs() * (height+1).abs()) as u64;
            // println!("{:?} - {:?} => {} x {} = {}\n", ne_tile, sw_tile, width, height, area);
            if area > max_area {
                max_area = area;
            }
        }
    }

    // NW and SE
    for nw_tile in quadrant_tiles[1].iter() {
        for se_tile in quadrant_tiles[2].iter() {
            let width = nw_tile.0 - se_tile.0;
            let height = nw_tile.1 - se_tile.1;
            let area = ((width+1).abs() * (height+1).abs()) as u64;
            // println!("{:?} - {:?} => {} x {} = {}\n", nw_tile, se_tile, width, height, area);
            if area > max_area {
                max_area = area;
            }
        }
    }

    Some(max_area)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
