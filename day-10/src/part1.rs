use crate::common::Grid;

pub fn run(input: &str) -> String {
    let grid = Grid::parse(input);
    let (main_loop, _) = grid.find_main_loop();
    let loop_length = main_loop.len();
    let furthest_tile_distance = loop_length / 2;
    return furthest_tile_distance.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"; // Add your test input here
        let expected_output = "4"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_run_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"; // Add your test input here
        let expected_output = "8"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}
