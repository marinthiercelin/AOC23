use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum TileValue {
    Empty,
    Start,
    Pipe(Direction, Direction),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

const ALL_DIRECTIONS: [Direction; 4] = [Direction::North, Direction::South, Direction::East, Direction::West];

#[derive(Debug)]
pub struct Tile {
    pub value: TileValue,
    pub position: Position,
}

pub type Position = (usize, usize);

pub struct Grid {
    pub start_tile_position: Position,
    pub tiles: Vec<Vec<Tile>>,
}

impl TileValue {
    fn parse(tile: &char) -> Self {
        match tile {
            '-' => TileValue::Pipe(Direction::East, Direction::West),
            '|' => TileValue::Pipe(Direction::North, Direction::South),
            'S' => TileValue::Start,
            'L' => TileValue::Pipe(Direction::North, Direction::East),
            'J' => TileValue::Pipe(Direction::North, Direction::West),
            'F' => TileValue::Pipe(Direction::South, Direction::East),
            '7' => TileValue::Pipe(Direction::South, Direction::West),
            '.' => TileValue::Empty,
            _ => panic!("Invalid tile value"),
        }
    }
}

impl Grid {
    pub fn parse(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut start_tile = None;
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, tile) in line.chars().enumerate() {
                let tile_value = TileValue::parse(&tile);
                let position = (x , y);
                let tile = Tile {
                    value: tile_value,
                    position,
                };
                if tile.value == TileValue::Start {
                    start_tile = Some(tile.position);
                }
                row.push(tile);
            }
            tiles.push(row);
        }
        Grid {
            start_tile_position: start_tile.unwrap(),
            tiles,
        }
    }

    fn get_tile(&self, position: Position) -> Option<&Tile> {
        self.tiles.get(position.1)?.get(position.0)
    }

    fn get_tile_in_direction(&self, position: Position, direction: &Direction) -> Option<&Tile> {
        match direction {
            Direction::North => if position.1 > 0 { self.get_tile((position.0, position.1 - 1)) } else { None },
            Direction::South => self.get_tile((position.0, position.1 + 1)),
            Direction::East => self.get_tile((position.0 + 1, position.1)),
            Direction::West => if position.0 > 0 { self.get_tile((position.0 - 1, position.1)) } else { None },
        }
    }

}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl Tile {
    fn get_next_direction(&self, current_direction: Direction) -> Direction {
        let previous_opposite = current_direction.opposite();
        if let TileValue::Pipe(direction1, direction2) = &self.value {
            if *direction1 == previous_opposite {
                return *direction2;
            } 
            if *direction2 == previous_opposite {
                return *direction1;
            } 
            panic!("Not a valid pipe for this origin direction {:?}", &self);
        }
        panic!("Not a pipe {:?}", &self);
    }
}

impl Grid {
    pub fn find_main_loop(&self) -> (Vec<&Tile>, Tile) {
        
        let start_tile = self.get_tile(self.start_tile_position).unwrap();

        let mut main_loop = vec![start_tile];

        let  (mut current_tile, mut current_direction) = self.find_start_neighbor();
        let start_direction = current_direction;
        loop {
            main_loop.push(current_tile);
            let next_direction = current_tile.get_next_direction(current_direction);
            let next_tile = self.get_tile_in_direction(current_tile.position, &next_direction).unwrap();
            current_tile = next_tile;
            current_direction = next_direction;
            if next_tile.value == TileValue::Start {
                break;
            }
        }
        let start_actual_tile = Tile { value: TileValue::Pipe(start_direction, current_direction.opposite()), ..*start_tile};
        (main_loop, start_actual_tile)
    }

    fn find_start_neighbor(&self) -> (&Tile, Direction) {
        for direction in ALL_DIRECTIONS {
            if let Some(neighbor_tile) = self.get_tile_in_direction(self.start_tile_position, &direction){
                if let TileValue::Pipe(direction1, direction2) = neighbor_tile.value {
                    let oposite_direction = direction.opposite();
                    if direction1 == oposite_direction || direction2 == oposite_direction {
                        return (neighbor_tile, direction);
                    }
                }
            }
        }
        panic!("No neighbor found for start tile");
    }
}

impl ToString for Grid {
    fn to_string(&self) -> String {
        let mut grid_string = String::new();
        for row in &self.tiles {
            for tile in row {
                grid_string.push_str(&tile.value.to_string());
            }
            grid_string.push_str("\n");
        }
        grid_string
    }
}

impl ToString for TileValue {
    fn to_string(&self) -> String {
        match self {
            TileValue::Empty => ".".to_string(),
            TileValue::Start => "S".to_string(),
            TileValue::Pipe(Direction::North, Direction::East) => "L".to_string(),
            TileValue::Pipe(Direction::North, Direction::West) => "J".to_string(),
            TileValue::Pipe(Direction::South, Direction::East) => "F".to_string(),
            TileValue::Pipe(Direction::South, Direction::West) => "7".to_string(),
            TileValue::Pipe(Direction::East, Direction::West) => "-".to_string(),
            TileValue::Pipe(Direction::North, Direction::South) => "|".to_string(),
            _ => panic!("Invalid tile value"),
        }
    }
}