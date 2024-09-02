use std::collections::HashMap;

use crate::common;
use crate::common::{Color, Game};


pub fn run(input: &str) -> String {
    let target_distribution = HashMap::from([(Color::Red, 12), (Color::Blue, 14), (Color::Green, 13)]);
    let games: Vec<_> = input.lines().map(|line| common::parse_game(line)).collect();
    let result = games.into_iter().filter(|game| game_is_possible(game, &target_distribution)).map(|game| game.number).sum::<u32>();
    return result.to_string();
}

// check if a game is possible given a target distribution
// to be possible a game must not have sets where the draw for a particular color
// exceeds the target distribution
fn game_is_possible(game: &Game, target_distribution: &HashMap<Color, u32>) -> bool {
    for set in &game.sets {
        for (color, number) in &set.draw {
            if target_distribution[color] < *number {
                return false;
            }
        }
    }
    true
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
        let expected = "8";
        let actual = run(input);
        assert_eq!(expected, actual)
    }
}