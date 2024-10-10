use std::collections::{HashMap, HashSet};

pub struct HeatMap {
    values: Vec<Vec<u32>>,
    dimensions: (usize, usize)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up, Down, Left, Right
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Node {
    row: usize,
    col: usize,
    dir: Direction,
}

pub struct MoveRules {
    pub min_blocks: u32,
    pub max_bocks: u32
}

const ALL_DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

impl HeatMap {
    pub fn parse(input: &str) -> Self {
        let values: Vec<Vec<u32>> = input.lines().map(|line|line.chars().map(|char| char.to_digit(10).unwrap()).collect()).collect();
        let dimensions = (values.len(), values[0].len());
        Self { values, dimensions }
    }

    fn edge_value(&self, node: &Node) -> u32 {
        self.values[node.row][node.col]
    }

    fn move_node(&self, node: &Node, dir: Direction, steps: usize) -> Option<Node> {
        let (max_row, max_column) = self.dimensions;
        return match (dir, node.row, node.col) {
            (Direction::Up, row, _) if row < steps => None,
            (Direction::Up, row, col) => Some(Node { row: row - steps, col, dir }),
            (Direction::Down, row, _) if row + steps >= max_row => None,
            (Direction::Down, row, col) => Some(Node { row: row + steps, col, dir }),
            (Direction::Left, _, col) if col < steps => None,
            (Direction::Left, row, col) => Some(Node { row, col: col - steps, dir }),
            (Direction::Right, _, col) if col + steps >= max_column => None,
            (Direction::Right, row, col) => Some(Node { row, col: col + steps, dir }),
        };
    }

    fn get_neighbors(&self, node: &Node, rules: &MoveRules) -> Vec<(Node, u32)> {
        let mut neighbors = Vec::new();
        for &dir in ALL_DIRECTIONS.iter().filter(|dir| !node.dir.eq(dir) && !node.dir.is_opposite(dir)) {
            let mut edge_cost = 0;
            for steps in 1..=rules.max_bocks {
                if let Some(neighbor) = self.move_node(node, dir, steps as usize) {
                    let edge_value =  self.edge_value(&neighbor);
                    edge_cost += edge_value;
                    if steps >= rules.min_blocks {
                        neighbors.push((neighbor, edge_cost));
                    }
                } else {
                    continue; // We hit the edge
                }
            }
        }
        // println!("Node: {:?} - Neighbors: {:?}", node, neighbors);
        neighbors
        
    }

    fn is_destination(&self, node: &Node) -> bool {
        let (max_row, max_col) = self.dimensions;
        node.row == max_row - 1 && node.col == max_col - 1
    }

    pub fn find_minimal_heat_loss(&self, rules: &MoveRules) -> u32 {
        let mut distances: HashMap<Node, u32> = HashMap::new();
        let mut queue = Vec::new();
        let mut predecessors: HashMap<Node, Node> = HashMap::new(); 
        [Direction::Down, Direction::Right].iter().for_each(|&dir| {
            let node = Node{row: 0, col: 0, dir};
            distances.insert(node.clone(), 0);
            queue.push(node);
        });
        // let mut visited: HashSet<Node> = HashSet::new(); 
        let mut destination_reached = 0;
        while !queue.is_empty() {
            println!("queue len: {}", queue.len());
            let index = queue.iter().enumerate().min_by(|(_,node1), (_, node2)| {
                distances.get(&node1).unwrap().cmp(distances.get(&node2).unwrap())
            }).map(|(index, _)| index).unwrap();
            let current_node = queue.remove(index);
            // visited.insert(current_node.clone());
            let current_dist = *distances.get(&current_node).unwrap();
            if self.is_destination(&current_node) {
                destination_reached += 1;
                println!("Destination reached: {}", destination_reached);
                if destination_reached == 2 {
                    break;
                }
            }
            for (neighbor, edge_cost) in self.get_neighbors(&current_node, rules) {
                // if visited.contains(&neighbor){
                //     continue;
                // }
                let new_dist = current_dist + edge_cost ;
                let old_dist = distances.get(&neighbor);
                if old_dist.is_none_or(|&old_dist| old_dist > new_dist ) {
                    distances.insert(neighbor.clone(), new_dist);
                    predecessors.insert(neighbor.clone(), current_node.clone());
                    queue.push(neighbor);
                }
            }
        } 
        let (node, min) = distances.iter().filter(|(node, _)| self.is_destination(node)).min_by(|(_, dist1), (_, dist2)| dist1.cmp(dist2)).expect("We should have reached the destination");
        // print_path(&predecessors, node, &distances);
        return *min;
    }
}

fn print_path(predecessors: &HashMap<Node, Node>, node: &Node, distances: &HashMap<Node, u32>) {
    let mut current_node = node;
    loop {
        println!("({},{}) {}",current_node.row, current_node.col, distances.get(current_node).unwrap());
        let predecessor = predecessors.get(current_node);
        if predecessor.is_none() {
            break;
        }
        current_node = predecessor.unwrap();
    }
}


impl Direction {
    fn is_opposite(&self, other: &Direction) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Down, Direction::Up) => true, 
            (Direction::Left, Direction::Right) => true,
            (Direction::Right, Direction::Left) => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_neighbors() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let heat_map = HeatMap::parse(&input);
        let node = Node{row: 0, col: 0, dir: Direction::Down};
        let rules = MoveRules{max_bocks: 3, min_blocks: 1};
        let neighbors = heat_map.get_neighbors(&node, &rules);
        println!("{:?}", neighbors)
    }
}