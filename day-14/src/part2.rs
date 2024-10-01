use std::{collections::HashMap, hash::{DefaultHasher, Hash, Hasher}};

use crate::common::{Direction, Platform};

pub fn run(input: &str) -> String {
    let mut platform = Platform::parse(input);
    let num_cycles = 1_000_000_000;
    
    let hash = |platform: &Platform| {
        let mut hasher = DefaultHasher::new();
        platform.to_string().hash(&mut hasher);
        hasher.finish()
    };
    let mut hashes = HashMap::new();
    hashes.insert(hash(&platform), 0);
    let mut repetition = None;
    for i in 0..num_cycles {
        platform = platform.cycle();
        let hash = hash(&platform);
        if hashes.contains_key(&hash) {
            repetition = Some((*hashes.get(&hash).unwrap(), i));
            break;
        };
        hashes.insert(hash, i);
    }
    if let Some((first, second)) = repetition {
       println!("Cycle {first} {second}");
       let number_of_cycles_to_repeat = second - first;
       let remaining_cycles = num_cycles - second;
       let remainder_after_cycle = remaining_cycles % number_of_cycles_to_repeat;
        for _ in 0..remainder_after_cycle-1 {
            platform = platform.cycle();
        }
    } 
    platform.load().to_string()
}

impl Platform {
    fn cycle(self) -> Self {
        let mut tilted = self;
        let all_directions = vec![Direction::North, Direction::West, Direction::South, Direction::East];
        for direction in all_directions {
            tilted = tilted.tilt(direction)
        }
        tilted
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let expected_output = "64";
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_cycle() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let expected_output = ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."; 
        let platform = Platform::parse(input);
        let cycled = platform.cycle();
        assert_eq!(cycled.to_string(), expected_output);
    }

    // Add more tests here
}