use std::vec;

pub fn run(input: &str) -> String {
    let universe = Universe::parse(input);
    let expanded_universe = universe.expand();
    let galaxy_positions = expanded_universe.get_galaxy_positions();
    let mut galaxy_distance_total = 0;
    for (galaxy_index, galaxy) in galaxy_positions.iter().enumerate() {
        for other_galaxy in galaxy_positions.iter().skip(galaxy_index + 1) {
            let distance = get_shortest_distance(galaxy, other_galaxy);
            galaxy_distance_total += distance;
        }
    }
    galaxy_distance_total.to_string()
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum UniversePoint {
    Empty,
    Galaxy
}

#[derive(Debug)]
struct Universe {
    grid: Vec<Vec<UniversePoint>>,
}

impl Universe {
    fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars().map(|c| {
                    match c {
                        '.' => UniversePoint::Empty,
                        '#' => UniversePoint::Galaxy,
                        _ => panic!("Invalid character in input")
                    }
                }).collect()
            })
            .collect();
        Self { grid }
    }

    fn expand(self) -> Self {
        let mut grid = self.grid;
        let empty_rows = grid.iter().enumerate().filter(|(_, row)| row.iter().all(|&point| point == UniversePoint::Empty)).map(|(index, _)| index).collect::<Vec<_>>();
        let empty_columns = (0..grid[0].len()).filter(|column| grid.iter().all(|row| row[*column] == UniversePoint::Empty)).collect::<Vec<_>>();
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

type GalaxyPosition = (usize, usize);

impl Universe {
    fn get_galaxy_positions(&self) -> Vec<GalaxyPosition> {
        self.grid.iter().enumerate().map(|(row_index, row)| {
            row.iter().enumerate().filter(|(_, &point)| point == UniversePoint::Galaxy).map(|(column_index, _)| (row_index, column_index)).collect::<Vec<_>>()
        }).flatten().collect()
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

fn get_shortest_distance(galaxy: &GalaxyPosition, other_galaxy: &GalaxyPosition) -> usize {
    let &(x1, y1) = galaxy;
    let &(x2, y2) = other_galaxy;
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs())as usize
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

    #[test]
    fn test_distance_function() {
        let galaxy1 = (6, 1);
        let galaxy2 = (11, 5);
        let distance = get_shortest_distance(&galaxy1, &galaxy2);
        assert_eq!(9, distance);
        let galaxy1 = (0, 4);
        let galaxy2 = (10, 10);
        let distance = get_shortest_distance(&galaxy1, &galaxy2);
        assert_eq!(15, distance);
        let galaxy1 = (2, 0);
        let galaxy2 = (8, 12);
        let distance = get_shortest_distance(&galaxy1, &galaxy2);
        assert_eq!(17, distance);
        let galaxy1 = (11, 0);
        let galaxy2 = (11, 6);
        let distance = get_shortest_distance(&galaxy1, &galaxy2);
        assert_eq!(5, distance);
    }

    // Add more tests here
}
