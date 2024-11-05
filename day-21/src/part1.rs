use itertools::Itertools;

pub fn run(input: &str) -> String {
    let map = Map::parse(input);
    map.count_reachable(64).to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    row: usize, col: usize
}

impl Position {
    fn get_neighbors(&self, (max_row, max_col): (usize, usize)) -> Vec<Self>{
        let mut neighbors = Vec::new();
        if self.row > 0 {
            neighbors.push(Position{
                row: self.row - 1, 
                col: self.col
            });
        }
        if self.row + 1 < max_row  {
            neighbors.push(Position{
                row: self.row + 1, 
                col: self.col
            });
        }
        if self.col > 0 {
            neighbors.push(Position{
                row: self.row, 
                col: self.col - 1
            });
        }
        if self.col + 1 < max_col  {
            neighbors.push(Position{
                row: self.row , 
                col: self.col + 1
            });
        }
        neighbors
    }
}

impl Tile {
    fn parse(char: &char) -> Self {
        match char {
            '.' => Self::Garden,
            '#' => Self::Rock,
            _ => panic!("Invalid tile: {char}")
        }
    }
}

struct Map {
    start: Position,
    dimensions: (usize, usize),
    tiles: Vec<Vec<Tile>>
}

impl Map {
    fn parse(input:&str) -> Self {
        let mut start = None;
        let tiles: Vec<Vec<Tile>> = input
            .lines()
            .enumerate()
            .map(|(row, line)|{
                line
                    .chars()
                    .enumerate()
                    .map(|(col, char)|{
                        if char == 'S' {
                            start = Some(Position { row, col });
                            Tile::Garden
                        } else {
                            Tile::parse(&char)
                        }
                    })
                    .collect()
            })
            .collect();
        let dimensions = (tiles.len(), tiles[0].len());
        Self { start: start.expect("Start not found"), tiles, dimensions}
    }

    fn count_reachable(&self, steps: u32) -> usize {
        let mut positions = vec![self.start];
        for _ in 0..steps {
            positions = positions.into_iter().flat_map(|pos|{
                pos.get_neighbors(self.dimensions).into_iter().filter(|pos|{
                    self.tiles[pos.row][pos.col] == Tile::Garden
                })
            }).unique().collect()
        }
        positions.len()
    }
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
        assert_eq!(map.count_reachable(6), expected_output);
    }
}
