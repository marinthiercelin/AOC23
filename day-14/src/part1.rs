use crate::common::{Direction, Platform};

pub fn run(input: &str) -> String {
    let platform = Platform::parse(input);
    let tilted = platform.tilt(Direction::North);
    tilted.load().to_string()
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
#OO..#...."; // Add your test input here
        let expected_output = "136"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }
}
