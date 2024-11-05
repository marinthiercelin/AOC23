use crate::common::Map;

pub fn run(input: &str) -> String {
    let map = Map::parse(input);
    map.count_reachable(64, false).to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "...........
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
        let map = Map::parse(input);
        let expected_output = 16;
        assert_eq!(map.count_reachable(6, false), expected_output);
    }
}
