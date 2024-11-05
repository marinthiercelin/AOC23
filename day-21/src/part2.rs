use crate::common::Map;

pub fn run(input: &str) -> String {
    let map = Map::parse(input);
    map.count_reachable(64, true).to_string()
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
        assert_eq!(map.count_reachable(6, true), expected_output);
    }

    #[test]
    fn test_run_10() {
        let map = Map::parse(INPUT);
        let expected_output = 50;
        assert_eq!(map.count_reachable(10, true), expected_output);
    }

    #[test]
    fn test_run_50() {
        let map = Map::parse(INPUT);
        let expected_output = 1594;
        let steps = 50;
        assert_eq!(map.count_reachable(steps, true), expected_output);
    }

    #[test]
    fn test_run_100() {
        let map = Map::parse(INPUT);
        let expected_output = 6536;
        let steps = 100;
        assert_eq!(map.count_reachable(steps, true), expected_output);
    }

    #[test]
    fn test_run_500() {
        let map = Map::parse(INPUT);
        let expected_output = 167004;
        let steps = 500;
        assert_eq!(map.count_reachable(steps, true), expected_output);
    }

    #[test]
    fn test_run_1000() {
        let map = Map::parse(INPUT);
        let expected_output = 668697;
        let steps = 1000;
        assert_eq!(map.count_reachable(steps, true), expected_output);
    }

    #[test]
    fn test_run_5000() {
        let map = Map::parse(INPUT);
        let expected_output = 16733044;
        let steps = 5000;
        assert_eq!(map.count_reachable(steps, true), expected_output);
    }
}