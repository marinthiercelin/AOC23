use std::env;

use day_15::part2;
use utils;

fn main() {
    let input = utils::read_input_file(env::args());
    let result = part2::run(&input);
    println!("Result: {result}");
}
