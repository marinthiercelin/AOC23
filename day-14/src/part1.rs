pub fn run(input: &str) -> String {
    let platform = Platform::parse(input);
    let tilted = platform.tilt(Direction::North);
    tilted.load().to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    RoundRock,
    CubeRock,
}

struct Platform {
    rows: Vec<Vec<Tile>>
}

impl Tile {
    fn parse(char: &char) -> Self {
        match char {
            'O' => Self::RoundRock,
            '#' => Self::CubeRock,
            '.' => Self::Empty,
            _ => panic!("Invalid value {char}")
        }
    }
}

impl Platform {
    fn parse(input: &str) -> Self {
        let rows = input.lines().map(|line| line.chars().map(|char| Tile::parse(&char)).collect()).collect();
        Self { rows }
    }
}

#[derive(PartialEq, Eq)]
enum Direction {
    North, South, West, East
}

impl Platform {

    fn columns(&self) -> Vec<Vec<Tile>> {
        (0..self.rows[0].len()).map(|column| {
            self.rows.iter().map(|row| row[column]).collect()
        }).collect()
    }

    fn from_columns(columns: &Vec<Vec<Tile>>) -> Self {
        let rows = (0..columns[0].len()).map(|row| {
            columns.iter().map(|column| column[row]).collect()
        }).collect();
        Self { rows }
    }

    fn tilt(self, direction: Direction) -> Self {
        let lines = if direction == Direction::West || direction == Direction::East {
            self.rows
        } else {
            self.columns()
        };
        let to_start = direction == Direction::West || direction == Direction::North;
        let new_lines = lines.iter().map(|line| tilt(line, to_start)).collect();
        if direction == Direction::West || direction == Direction::East {
            Self { rows: new_lines }
        } else {
            Self::from_columns(&new_lines)
        }
    }
}

impl Tile {
    fn as_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::CubeRock => '#',
            Self::RoundRock => 'O',
        }
    }
}

impl ToString for Platform {
    fn to_string(&self) -> String {
        let rows: Vec<String> = self.rows.iter().map(|row| row.iter().map(|tile| tile.as_char()).collect::<String>()).collect();
        rows.join("\n")
    }
}

fn tilt(line: &Vec<Tile>, to_start: bool) -> Vec<Tile> {
    let mut new_line = line.clone();
    let mut tiles: Box<dyn DoubleEndedIterator<Item = (usize, &Tile)>> = Box::new(line.iter().enumerate());
    if !to_start {
        tiles = Box::new(tiles.rev());
    }
    for (index, tile) in tiles {
        if *tile == Tile::RoundRock {
            let mut new_position = None;
            let scan_range: Box<dyn DoubleEndedIterator<Item = usize>> = if to_start {
                Box::new((0..index).rev())
            } else {
                Box::new(index+1..line.len())
            };
            for scan_index in  scan_range {
                if new_line[scan_index] != Tile::Empty {
                    new_position = Some(scan_index + 1);
                    break;
                }
            }
            let new_position = new_position.unwrap_or(0);
            if new_position != index {
                new_line[index] = Tile::Empty;
                new_line[new_position] = Tile::RoundRock;
            }
        }
    }
    new_line
}

impl Platform {
    fn load(&self) -> usize {
        let mut load = 0;
        let len = self.rows.len();
        for (row_index, row) in self.rows.iter().enumerate() {
            for tile in row {
                if *tile == Tile::RoundRock {
                    load += len - row_index;
                }
            }
        }
        load
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
#OO..#...."; // Add your test input here
        let expected_output = "136"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_tilt_north() {
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
        let expected_output = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."; // Add the expected output here
        let platform = Platform::parse(input);
        let tilted = platform.tilt(Direction::North);
        assert_eq!(tilted.to_string(), expected_output);
    }

    #[test]
    fn test_load() {
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
        let expected_output = 136; // Add the expected output here
        let platform = Platform::parse(input);
        let tilted = platform.tilt(Direction::North);
        assert_eq!(tilted.load(), expected_output);
    }

    // Add more tests here
}
