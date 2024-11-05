use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    row: i32, col: i32
}

impl Position {
    fn get_neighbors(&self, (max_row, max_col): (i32, i32), repeat: bool) -> Vec<Self>{
        let mut neighbors = Vec::new();
        if self.row > 0 || repeat {
            neighbors.push(Position{
                row: self.row - 1, 
                col: self.col
            });
        }
        if self.row + 1 < max_row  || repeat {
            neighbors.push(Position{
                row: self.row + 1, 
                col: self.col
            });
        } 
        if self.col > 0 || repeat {
            neighbors.push(Position{
                row: self.row, 
                col: self.col - 1
            });
        }
        if self.col + 1 < max_col  || repeat {
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

pub struct Map {
    start: Position,
    dimensions: (i32, i32),
    tiles: Vec<Vec<Tile>>
}

impl Map {
    pub fn parse(input:&str) -> Self {
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
                            start = Some(Position { row: row as i32, col: col as i32 });
                            Tile::Garden
                        } else {
                            Tile::parse(&char)
                        }
                    })
                    .collect()
            })
            .collect();
        let dimensions = (tiles.len() as i32, tiles[0].len() as i32);
        Self { start: start.expect("Start not found"), tiles, dimensions}
    }

    fn get_tile(&self, pos: &Position) -> Tile {
        let (max_row, max_col) = self.dimensions;
        let row = pos.row.rem_euclid(max_row) as usize;
        let col = pos.col.rem_euclid(max_col) as usize;
        self.tiles[row][col]
    }

    pub fn count_reachable(&self, steps: u32, repeat: bool) -> usize {
        let mut positions = vec![self.start];
        for _ in 0..steps {
            positions = positions.into_iter().flat_map(|pos|{
                pos.get_neighbors(self.dimensions, repeat).into_iter().filter(|pos|{
                    self.get_tile(&pos) == Tile::Garden
                })
            }).unique().collect()
        }
        positions.len()
    }
}