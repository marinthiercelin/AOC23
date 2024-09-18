use std::collections::HashMap;

use crate::common::{Direction, Grid, Position, Tile, TileValue};

pub fn run(input: &str) -> String {
    let grid = Grid::parse(input);
    let dimension = (grid.tiles[0].len(), grid.tiles.len());
    let (mut main_loop, start_actual_direction) = grid.find_main_loop();
    main_loop[0] = &start_actual_direction;
    let enclosed_tiles = find_enclosed_tiles(dimension, main_loop);
    println!("{}", grid.display_enclosed_tiles(&enclosed_tiles));
    enclosed_tiles.len().to_string()
}

impl Grid {
    fn display_enclosed_tiles(&self, enclosed_tiles: &Vec<Position>) -> String {
        let mut grid_string = String::new();
        for y in 0..self.tiles.len() {
            for x in 0..self.tiles[0].len() {
                let position = (x, y);
                if enclosed_tiles.contains(&position) {
                    grid_string.push_str("I");
                } else {
                    grid_string.push_str(&self.tiles[y][x].value.to_string());
                }
            }
            grid_string.push_str("\n");
        }
        grid_string
    }
}

fn find_enclosed_tiles(grid_dimension: (usize, usize), main_loop: Vec<&Tile>) -> Vec<Position> {
    let loop_by_positions = main_loop.iter().map(|&tile| (tile.position, tile)).collect::<HashMap::<Position, &Tile>>();
    let mut enclosed_tiles = Vec::new();
    for y in 0..grid_dimension.1 {
        for x in 0..grid_dimension.0 {
            let position = (x, y);
            if loop_by_positions.contains_key(&position) {
                continue;
            }
            let mut ray_tiles = Vec::new();
            for ray_index in x+1..grid_dimension.0 {
                let ray_position = (ray_index, y);
                if loop_by_positions.contains_key(&ray_position) {
                    let ray_tile = loop_by_positions.get(&ray_position).unwrap();
                    ray_tiles.push(ray_tile);
                }
            }
            let mut cross_count = 0;
            let mut incomplete_cross = None;
            for ray_tile in ray_tiles {
                if ray_tile.value == TileValue::Pipe(Direction::North, Direction::South) {
                    cross_count += 1;
                }
                if ray_tile.value == TileValue::Pipe(Direction::North, Direction::East) || ray_tile.value == TileValue::Pipe(Direction::South, Direction::East) {
                    incomplete_cross = Some(&ray_tile.value)
                }
                if ray_tile.value == TileValue::Pipe(Direction::North, Direction::West) {
                    if *incomplete_cross.unwrap() ==  TileValue::Pipe(Direction::South, Direction::East) {
                        cross_count += 1;
                    }
                    incomplete_cross = None;
                }
                if ray_tile.value == TileValue::Pipe(Direction::South, Direction::West) {
                    if *incomplete_cross.unwrap() ==  TileValue::Pipe(Direction::North, Direction::East) {
                        cross_count += 1;
                    }
                    incomplete_cross = None;
                }
            }
            if cross_count % 2 == 1 {
                enclosed_tiles.push(position);
            }
        }

    }
    enclosed_tiles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"; // Add your test input here
        let expected_output = "10"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_run_2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."; // Add your test input here
        let expected_output = "4"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}