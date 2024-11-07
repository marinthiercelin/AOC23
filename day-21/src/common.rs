use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Garden,
    Rock
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub row: usize, pub col: usize
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up, Down, Left, Right
}

pub const ALL_DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

impl Map {
    pub fn move_to(&self, position: &Position, direction: &Direction) -> Option<Position> {
        let (max_row, max_col) = self.dimensions;
        let mut row = position.row as i32;
        let mut col = position.col as i32;
        match direction {
            Direction::Up => row -= 1,
            Direction::Down => row += 1,
            Direction::Left => col -= 1,
            Direction::Right => col += 1,
        }
        if row < 0 || row >= max_row as i32 || col < 0 || col >= max_col as i32 {
            return None
        }
        return Some( Position { row: row as usize, col: col as usize} )
    }
}

impl Map {
    pub fn get_neighbors(&self, position: &Position) -> Vec<Position> {
        let neighbors = ALL_DIRECTIONS
        .iter()
        .filter_map(|direction| self.move_to(position, direction))
        .filter(|position| self.get_tile(position) == Tile::Garden);
        return neighbors.collect();
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
    pub start: Position,
    pub dimensions: (usize, usize),
    pub tiles: Vec<Vec<Tile>>
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

    fn get_tile(&self, pos: &Position) -> Tile {
        self.tiles[pos.row][pos.col]
    }

    pub fn count_reachable(&self, steps: u32) -> usize {
        let mut positions = vec![self.start];
        for _ in 0..steps {
            positions = positions.into_iter().flat_map(|pos|{
                self.get_neighbors(&pos).into_iter().filter(|pos|{
                    self.get_tile(&pos) == Tile::Garden
                })
            }).unique().collect()
        }
        positions.len()
    }
}