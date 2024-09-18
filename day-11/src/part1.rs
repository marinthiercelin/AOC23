use std::vec;

use crate::common::{self, Universe, UniversePoint};

pub fn run(input: &str) -> String {
    let universe = Universe::parse(input);
    let expanded_universe = universe.expand();
    let galaxy_positions = expanded_universe.get_galaxy_positions();
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

    fn expand(self) -> Self {
        let empty_rows = self.get_empty_rows();
        let empty_columns = self.get_empty_columns();
        let mut grid = self.grid;
        for (empty_columns_already_inserted, empty_column) in empty_columns.into_iter().enumerate() {
            for row in &mut grid {
                row.insert(empty_column + empty_columns_already_inserted , UniversePoint::Empty);
            }
        }
        let row_length = grid[0].len();
        for (empty_rows_already_inserted, empty_row) in empty_rows.into_iter().enumerate() {
            grid.insert(empty_row + empty_rows_already_inserted, vec![UniversePoint::Empty; row_length]);
        }
        Self { grid }
    }
}

impl ToString for Universe {
    fn to_string(&self) -> String {
        self.grid.iter().map(|row| {
            row.iter().map(|point| {
                match point {
                    UniversePoint::Empty => '.',
                    UniversePoint::Galaxy => '#'
                }
            }).collect::<String>()
        }).collect::<Vec<_>>().join("\n")
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
        let expected_output = "374"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_expand_function() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let expected_output = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
        let universe = Universe::parse(input);
        let expanded_universe = universe.expand();
        let actual_output = expanded_universe.to_string();
        assert_eq!(expected_output, actual_output);
    }

    // Add more tests here
}
