use std::collections::{HashMap, VecDeque};

use crate::common::{Map, Position};

pub fn run(input: &str) -> String {
    let map = Map::parse(input);
    let steps = 26_501_365;
    map.find_reachable(steps).to_string()
}

impl Map {
    fn find_reachable(&self, steps: usize) -> usize {

        let shortest_distances = self.get_shortest_distance();
        let (height, width) = self.dimensions;
        assert!(height == width);
        let half_grid = height / 2;
        assert!(steps % height == half_grid);
        let good_parity = steps % 2;
        let good_parity_tiles = shortest_distances.iter().filter(|(_, &distance)| distance % 2 == good_parity).count();
        let bad_parity_tiles = shortest_distances.iter().filter(|(_, &distance)| distance % 2 != good_parity).count();
        let good_parity_corner_tiles = shortest_distances.iter().filter(|(_, &distance)| distance % 2 == good_parity && distance > half_grid).count();
        let bad_parity_corner_tiles = shortest_distances.iter().filter(|(_, &distance)| distance % 2 != good_parity && distance > half_grid).count();
        
        let n = (steps - half_grid) / height;

        let n_square = n.pow(2);
        let n_plus_one_square = (n + 1).pow(2);
        let (good_parity_multiplicator, bad_parity_multiplicator) = if n % 2 == 0 {
            (n_plus_one_square, n_square)
        } else {
            (n_square, n_plus_one_square)
        };

        let mut total = 
            good_parity_multiplicator * good_parity_tiles
            + bad_parity_multiplicator * bad_parity_tiles;

        if n % 2 == 0{
            total -= (n+1) * good_parity_corner_tiles;
            total += n * bad_parity_corner_tiles;
        } else {
            total -= (n+1) * bad_parity_corner_tiles;
            total += n * good_parity_corner_tiles;
        };

        return total;
    }

    fn get_shortest_distance(&self) -> HashMap<Position, usize> {
        let mut visited = HashMap::new();

        let mut queue = VecDeque::new();

        queue.push_back((self.start, 0));

        while !queue.is_empty() {
            let (position, distance) = queue.pop_front().unwrap();
            
            if visited.contains_key(&position) {
                continue;
            }

            visited.insert(position, distance);

            for neighbor in self.get_neighbors(&position) {
                if visited.contains_key(&neighbor) {
                    continue;
                }
                queue.push_back((neighbor, distance + 1));
            }
        }

        return visited;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_run_6() {
        let map = Map::parse(INPUT);
        let expected_output = 16;
        assert_eq!(map.find_reachable(6), expected_output);
    }

    #[test]
    fn test_run_10() {
        let map = Map::parse(INPUT);
        let expected_output = 50;
        assert_eq!(map.find_reachable(10), expected_output);
    }

    #[test]
    fn test_run_50() {
        let map = Map::parse(INPUT);
        let expected_output = 1594;
        let steps = 50;
        assert_eq!(map.find_reachable(steps), expected_output);
    }

    #[test]
    fn test_run_100() {
        let map = Map::parse(INPUT);
        let expected_output = 6536;
        let steps = 100;
        assert_eq!(map.find_reachable(steps), expected_output);
    }

    #[test]
    fn test_run_500() {
        let map = Map::parse(INPUT);
        let expected_output = 167004;
        let steps = 500;
        assert_eq!(map.find_reachable(steps), expected_output);
    }

    #[test]
    fn test_run_1000() {
        let map = Map::parse(INPUT);
        let expected_output = 668697;
        let steps = 1000;
        assert_eq!(map.find_reachable(steps), expected_output);
    }

    #[test]
    fn test_run_5000() {
        let map = Map::parse(INPUT);
        let expected_output = 16733044;
        let steps = 5000;
        assert_eq!(map.find_reachable(steps), expected_output);
    }
}