advent_of_code::solution!(8);

#[derive(Debug)]
struct Point(i64, i64, i64);

#[derive(Debug, Clone)]
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
    // Problem description looks suspiciously like kruskal's algorithm
    // We're looking for the last link made, i.e. the longest edge in the MST
    // Kruskals isn't ideal for a connected graph, so lets use prim's
    let points: Vec<Point> = input.lines().map(Point::from_str).collect();
    let n = points.len();

    // Prim's algorithm initialization
    let mut mst_nodes: Vec<bool> = vec![false; n];          // Nodes in the MST
    let mut shortest_link: Vec<u64> = vec![u64::MAX; n];    // Shortest link to MST
    let mut parent: Vec<usize> = vec![0; n];                // Closest node in MST

    // Track the longest edge added to the MST
    let mut longest_edge = Edge(0, 0, 0);

    // Start from node 0
    mst_nodes[0] = true; // Start from node 0
    let mut nodes_in_mst = 1; // Count of nodes in MST to know when we're done

    // Initialize shortest links from node 0
    for i in 1..n {
        shortest_link[i] = points[0].dist_sq(&points[i]);
        parent[i] = 0;
    }

    for _ in 1..n {
        // Find the minimum edge connecting a node to the MST
        let mut min_dist = u64::MAX;
        let mut next_node = 0;
        for i in 0..n {
            if !mst_nodes[i] && shortest_link[i] < min_dist {
                min_dist = shortest_link[i];
                next_node = i;
            }
        }

        // Add this node to the MST
        mst_nodes[next_node] = true;
        nodes_in_mst += 1;

        // Check if this edge is the longest so far
        if min_dist > longest_edge.2 {
            longest_edge = Edge(parent[next_node], next_node, min_dist);
        }

        // If we've added all nodes, we're done
        if nodes_in_mst == n { break; }

        // Update shortest links for remaining nodes
        for i in 0..n {
            if !mst_nodes[i] {
                let dist = points[next_node].dist_sq(&points[i]);
                if dist < shortest_link[i] {
                    shortest_link[i] = dist;
                    parent[i] = next_node;
                }
            }
        }
    }

    let longest = longest_edge;
    Some((points[longest.0].0 * points[longest.1].0) as u64)
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
