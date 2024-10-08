use std::collections::HashSet;

pub struct Contraption {
    pub tiles: Vec<Vec<Tile>>
}

pub enum Tile {
    Empty,
    Vertical,
    Horizontal,
    Slash,
    BackSlash
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct BeamHead {
    pub position: (usize, usize),
    pub direction: Direction
}

impl Tile {
    fn parse(char: char) -> Self {
        match char {
            '.' => Self::Empty,
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            '/' => Self::Slash,
            '\\' => Self::BackSlash,
            _ => panic!("Invalid tile {char}")
        }
    }

    fn affect_direction(&self, direction: &Direction) -> Vec<Direction> {
        match (self, direction) {
            (Self::Empty, _) => vec![*direction],
            
            (Self::Horizontal, Direction::Left | Direction::Right) => vec![*direction],
            (Self::Horizontal, Direction::Up | Direction::Down) => vec![Direction::Left, Direction::Right],
            
            (Self::Vertical, Direction::Up | Direction::Down) => vec![*direction],
            (Self::Vertical, Direction::Right | Direction::Left) => vec![Direction::Up, Direction::Down],
            
            (Self::BackSlash, Direction::Down) => vec![Direction::Right],
            (Self::BackSlash, Direction::Right) => vec![Direction::Down],
            (Self::BackSlash, Direction::Up) => vec![Direction::Left],
            (Self::BackSlash, Direction::Left) => vec![Direction::Up],
            
            (Self::Slash, Direction::Down) => vec![Direction::Left],
            (Self::Slash, Direction::Left) => vec![Direction::Down],
            (Self::Slash, Direction::Up) => vec![Direction::Right],
            (Self::Slash, Direction::Right) => vec![Direction::Up],
        }
    }

}

impl Contraption {
    pub fn parse(input: &str) -> Self {
        let tiles = input
        .lines()
        .map(|line| line.chars().map(|char|Tile::parse(char)).collect())
        .collect();
        Self { tiles }
    }

    pub fn energize(&self, original_beam: BeamHead) -> Vec<(usize, usize)> {
        let mut working_beams: Vec<BeamHead> = Vec::new();
        let mut past_beams = HashSet::new();
        let bounds = (self.tiles.len(), self.tiles[0].len());
        past_beams.insert(original_beam.clone());
        working_beams.push(original_beam);
        while working_beams.len() > 0 {
            let beam = working_beams.pop().unwrap();
            let tile = &self.tiles[beam.position.0][beam.position.1];
            for new_direction in tile.affect_direction(&beam.direction) {
                let new_beam = beam.extend(new_direction, bounds);
                if let Some(new_beam) = new_beam {
                    if !past_beams.contains(&new_beam) {
                        past_beams.insert(new_beam.clone());
                        working_beams.push(new_beam);
                    }
                }
            }
        }
        let energized_positions = past_beams.into_iter().map(|beam| beam.position).collect::<HashSet<_>>().into_iter().collect();
        energized_positions
    }
}

impl BeamHead {
    fn extend(&self, new_direction: Direction, bounds: (usize, usize)) -> Option<Self> {
        let (max_row, max_column) = bounds;
        let (row, column) = self.position;
        let new_position = match (new_direction, row, column) {
            (Direction::Down, row, _) if row == max_row - 1 => None,
            (Direction::Down, row, column) => Some((row + 1, column)),
            (Direction::Up, 0, _) => None,
            (Direction::Up, row, column) => Some((row - 1, column)),
            (Direction::Right, _, column) if column == max_column - 1 => None,
            (Direction::Right, row, column) => Some((row, column + 1)),
            (Direction::Left, _, 0) => None,
            (Direction::Left, row, column) => Some((row, column - 1)),
        };
        new_position.map(|new_position| {
            Self{position: new_position, direction: new_direction}
        })
    }
}