advent_of_code::solution!(8);

struct Point(i64, i64, i64);
struct Edge(usize, usize, u64);

impl Point {
    fn from_str(s: &str) -> Self {
        let mut coords = s
            .split(',')
            .map(|x| x.trim().parse().unwrap());
        Point(
            coords.next().unwrap(), 
            coords.next().unwrap(), 
            coords.next().unwrap()
        )
    }

    fn dist_sq(&self, other: &Point) -> u64 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        let dz = self.2 - other.2;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let points: Vec<Point> = input.lines().map(Point::from_str).collect();

    // grumblegrumble arbitrary value in problem statement
    let connections = match points.len() {
        20 => 10,  // Example input has 20 points and wants 10 iterations
        _ => 1000, // Actual input wants 1000 iterations, no assumptions about length
    };

    // Classic find the n minimum values problem
    // We could take some shortcuts but it's probably not worth it
    // since the distance calculation is pretty cheap and doesn't branch
    // Let's just iterate through all pairs and insertion sort into a fixed-size array
    let mut closest_pairs: Vec<Edge> = Vec::with_capacity(connections);
    let mut threshold_dist_sq = u64::MAX;
    // Iterate pairs
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let dist_sq = points[i].dist_sq(&points[j]);
            if dist_sq < threshold_dist_sq {
                // Insert into sorted list
                let insert_pos = closest_pairs
                    .binary_search_by(|edge| edge.2.cmp(&dist_sq))
                    .unwrap_or_else(|e| e);
                // Pre-emptively remove the last edge if at capacity
                if closest_pairs.len() == connections {
                    closest_pairs.pop();
                }
                // Insert the new edge
                closest_pairs.insert(insert_pos, Edge(i, j, dist_sq));
                // Update threshold
                if closest_pairs.len() == connections {
                    threshold_dist_sq = closest_pairs.last().unwrap().2;
                }
            }
        }
    }

    // Now we need to connect the junction boxes into circuits
    // Let's initialize each point as its own circuit
    let mut circuits: Vec<usize> = (0..points.len()).collect();
    // And also add a helper to count the values in a circuit
    let mut circuit_sizes: Vec<usize> = vec![1; points.len()];

    // Now iterate through the pairs and merge the circuits
    for edge in closest_pairs {
        let circuit_a = circuits[edge.0];
        let circuit_b = circuits[edge.1];
        if circuit_a != circuit_b {
            // Put both nodes in the smaller-valued circuit
            let new_circuit = circuit_a.min(circuit_b);
            let old_circuit = circuit_a.max(circuit_b);
            for i in 0..circuits.len() {
                if circuits[i] == old_circuit {
                    circuits[i] = new_circuit;
                    circuit_sizes[new_circuit] += 1;
                }
            }
            // println!("{} <==> {} | {}: {}",
            //     circuit_a, circuit_b, new_circuit, circuit_sizes[new_circuit]
            // );
        }
    }

    // Now find the 3 largest circuits and return the product of their sizes
    circuit_sizes.sort_unstable();
    // print!("Sizes: {:?}", circuit_sizes);
    Some(
        circuit_sizes
            .iter()
            .rev()
            .take(3)
            .map(|&size| size as u64)
            .product()
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    // _Fine_ I'll use the MB of RAM to store a distance matrix
    let points: Vec<Point> = input.lines().map(Point::from_str).collect();
    let n = points.len();
    let mut dist_matrix: Vec<u64> = vec![0; n*n];
    for i in 0..n {
        for j in 0..n {
            dist_matrix[i*n + j] = points[i].dist_sq(&points[j]);
        }
    }

    println!("Distance matrix calculated.");

    // This time we jump straight to circuit merging
    let mut circuits: Vec<usize> = (0..points.len()).collect();
    let mut circuit_sizes: Vec<usize> = vec![1; points.len()];
    let mut max_circuit_size = 1;
    let mut answer = 0;
    while max_circuit_size < n {
        // find the minimum distance in the distance matrix
        let min_idx = dist_matrix.iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();
        // Calculate the node indices
        let (i, j) = (min_idx / n, min_idx % n);
        // print!("{} <==> {} ({}) | ",
        //         i, j, dist_matrix[min_idx]
        //     );
        // Set that distance to max so we don't find it again
        dist_matrix[min_idx] = u64::MAX;
        // Merge the circuits if they are different
        let circuit_a = circuits[i];
        let circuit_b = circuits[j];
        if circuit_a != circuit_b {
            // This is a new connection, track it as the answer
            answer = (points[i].0*points[j].0) as u64;
            let new_circuit = circuit_a.min(circuit_b);
            let old_circuit = circuit_a.max(circuit_b);
            for k in 0..circuits.len() {
                if circuits[k] == old_circuit {
                    circuits[k] = new_circuit;
                    circuit_sizes[new_circuit] += 1;
                }
            }
            if circuit_sizes[new_circuit] > max_circuit_size {
                max_circuit_size = circuit_sizes[new_circuit];
                println!("New max circuit size: {}", max_circuit_size);
            }
            println!("{} <==> {} | {}: {}",
                circuit_a, circuit_b, new_circuit, circuit_sizes[new_circuit]
            );
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
