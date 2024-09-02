use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Color {
    Red,
    Blue,
    Green,
}

pub struct Game {
    pub(crate) number: u32,
    pub(crate) sets: Vec<Set>,
}

pub struct Set {
    pub(crate) draw: HashMap<Color, u32>,
}

// parse the line "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" so that it returns a Game struct
pub fn parse_game(line: &str) -> Game {
    let mut parts = line.split(": ");
    let number = parts
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let sets = parts
        .next()
        .unwrap()
        .split(";")
        .map(|set| parse_set(set.trim()))
        .collect();
    Game { number, sets }
}

fn parse_set(set: &str) -> Set {
    let mut draw = HashMap::new();
    for item in set.split(", ") {
        let mut parts = item.split(" ");
        let number = parts.next().unwrap().parse::<u32>().unwrap();
        let color = match parts.next().unwrap() {
            "red" => Color::Red,
            "blue" => Color::Blue,
            "green" => Color::Green,
            _ => panic!("Invalid color"),
        };
        draw.insert(color, number);
    }
    Set { draw }
}
