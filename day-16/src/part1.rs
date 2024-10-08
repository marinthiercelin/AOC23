use std::collections::HashSet;

pub fn run(input: &str) -> String {
    let contraption = Contraption::parse(input);
    let energized = contraption.energize();
    energized.len().to_string()
}

struct Contraption {
    tiles: Vec<Vec<Tile>>
}

enum Tile {
    Empty,
    Vertical,
    Horizontal,
    Slash,
    BackSlash
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct BeamHead {
    position: (usize, usize),
    direction: Direction
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
    fn parse(input: &str) -> Self {
        let tiles = input
        .lines()
        .map(|line| line.chars().map(|char|Tile::parse(char)).collect())
        .collect();
        Self { tiles }
    }

    fn energize(&self) -> Vec<(usize, usize)> {
        let mut working_beams: Vec<BeamHead> = Vec::new();
        let original_beam = BeamHead{position: (0,0), direction: Direction::Right};
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let expected_output = "46";
        assert_eq!(run(input), expected_output);
    }
}
