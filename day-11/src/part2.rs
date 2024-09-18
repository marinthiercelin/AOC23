use crate::common::{self, GalaxyPosition, Universe, UniversePoint};

pub fn run(input: &str) -> String {
    run_with_factor(input, 1_000_000)
}

fn run_with_factor(input: &str, factor: usize) -> String {
    let universe = Universe::parse(input);
    let galaxy_positions = universe.expand_large(factor);
    let mut galaxy_distance_total = 0;
    for (galaxy_index, galaxy) in galaxy_positions.iter().enumerate() {
        for other_galaxy in galaxy_positions.iter().skip(galaxy_index + 1) {
            let distance = common::get_shortest_distance(galaxy, other_galaxy);
            galaxy_distance_total += distance;
        }
    }
    galaxy_distance_total.to_string()
}

impl Universe {
    fn expand_large(&self, factor: usize) -> Vec<GalaxyPosition> {
        let empty_rows = self.get_empty_rows();
        let empty_columns = self.get_empty_columns();
        let galaxy_positions = self.grid.iter().enumerate().map(|(row_index, row)| {
            row.iter().enumerate().filter(|(_, &point)| point == UniversePoint::Galaxy).map(|(column_index, _)| (row_index, column_index)).collect::<Vec<_>>()
        }).flatten().collect::<Vec<_>>();
        let expanded_galaxy_positions = galaxy_positions.iter().map(|(row, column)| {
            let empty_rows_before = empty_rows.iter().filter(|&empty_row| empty_row < row).count();
            let empty_columns_before = empty_columns.iter().filter(|&empty_column| empty_column < column).count();
            (row + empty_rows_before * (factor - 1), column + empty_columns_before * (factor- 1))
        }).collect();
        expanded_galaxy_positions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."; // Add your test input here
        let expected_output = "1030"; // Add the expected output here
        assert_eq!(run_with_factor(input, 10), expected_output);
    }

    #[test]
    fn test_run_100() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."; // Add your test input here
        let expected_output = "8410"; // Add the expected output here
        assert_eq!(run_with_factor(input, 100), expected_output);
    }

    // Add more tests here
}