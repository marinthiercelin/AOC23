use std::collections::{HashMap, HashSet};

pub fn run(input: &str) -> String {
    let heat_map = HeatMap::parse(input);
    let minimal_heat_loss = heat_map.find_minimal_heat_loss();
    minimal_heat_loss.to_string()
}

struct HeatMap {
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
    last_turn: u8
}

const ALL_DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

impl HeatMap {
    fn parse(input: &str) -> Self {
        let values: Vec<Vec<u32>> = input.lines().map(|line|line.chars().map(|char| char.to_digit(10).unwrap()).collect()).collect();
        let dimensions = (values.len(), values[0].len());
        Self { values, dimensions }
    }

    fn edge_value(&self, node: &Node) -> u32 {
        self.values[node.row][node.col]
    }

    fn move_node(&self, node: &Node, dir: Direction) -> Option<Node> {
        let (max_row, max_column) = self.dimensions;
        let last_turn = if dir == node.dir {
            node.last_turn + 1
        } else {
            0
        };
        if last_turn > 2 {
            return None;
        }
        if dir.is_opposite(&node.dir) {
            return None;
        }
        return match (dir, node.row, node.col) {
            (Direction::Up, 0, _) => None,
            (Direction::Up, row, col) => Some(Node { row: row - 1, col, dir, last_turn }),
            (Direction::Down, row, _) if row + 1 == max_row => None,
            (Direction::Down, row, col) => Some(Node { row: row + 1, col, dir, last_turn }),
            (Direction::Left, _, 0)=> None,
            (Direction::Left, row, col) => Some(Node { row, col: col - 1, dir, last_turn }),
            (Direction::Right, _, col) if col + 1 == max_column => None,
            (Direction::Right, row, col) => Some(Node { row, col: col + 1, dir, last_turn }),
        };
    }

    fn get_neighbors(&self, node: &Node) -> Vec<(Node, u32)> {
        let mut edges = Vec::new();
        for dir in ALL_DIRECTIONS {
            if let Some(neighbor) = self.move_node(node, dir) {
                let edge_value =  self.edge_value(&neighbor);
                edges.push((neighbor, edge_value));
            }
        }
        edges
    }

    fn is_destination(&self, node: &Node) -> bool {
        let (max_row, max_col) = self.dimensions;
        node.row == max_row - 1 && node.col == max_col - 1
    }

    fn find_minimal_heat_loss(&self) -> u32 {
        let mut distances: HashMap<Node, u32> = HashMap::new();
        let mut queue = Vec::new();
        [Direction::Down, Direction::Right].iter().for_each(|&dir| {
            let node = Node{row: 0, col: 0, dir, last_turn: 0};
            distances.insert(node.clone(), 0);
            queue.push(node);
        });
        while !queue.is_empty() {
            let index = queue.iter().enumerate().min_by(|(_,node1), (_, node2)| {
                distances.get(&node1).unwrap().cmp(distances.get(&node2).unwrap())
            }).map(|(index, _)| index).unwrap();
            let current_node = queue.remove(index);
            let current_dist = *distances.get(&current_node).unwrap();
            for (neighbor, edge_cost) in self.get_neighbors(&current_node) {
                let new_dist = current_dist + edge_cost ;
                let old_dist = distances.get(&neighbor);
                if old_dist.is_none_or(|&old_dist| old_dist > new_dist ) {
                    distances.insert(neighbor.clone(), new_dist);
                    queue.push(neighbor);
                }
            }
        } 
        let min = distances.into_iter().filter(|(node, _)| self.is_destination(node)).map(|(_, distance)| distance).min();
        return min.expect("We should have reached the destination");
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
    fn test_run() {
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
        let expected_output = "102";
        assert_eq!(run(input), expected_output);
    }
}
