use std::collections::HashMap;

use crate::common::{self, Color, Game};

pub fn run(input: &str) -> String {
    let games: Vec<_> = input.lines().map(|line| common::parse_game(line)).collect();
    let result: u32 = games
    .into_iter()
    .map(|game| get_minimal_distribution(game))
    .map(|distribution| distribution.into_values().fold(1, |acc, x| acc * x))
    .sum();
    return result.to_string();
}

// Get the max value of each color seen in one of the sets of the game
fn get_minimal_distribution(game: Game) -> HashMap<Color, u32> {
    let mut distribution = HashMap::new();
    for set in game.sets {
        for (color, number) in set.draw {
            let current = distribution.entry(color).or_insert(0);
            if number > *current {
                *current = number;
            }
        }
    }
    distribution
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn run_test_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = "2286";
        let actual = run(input);
        assert_eq!(expected, actual)
    }
}