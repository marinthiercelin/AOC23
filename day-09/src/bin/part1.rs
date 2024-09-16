use std::env;

use day_09::part1;
use utils;

fn main() {
    let input = utils::read_input_file(env::args());
    let result = part1::run(&input);
    println!("Result: {result}");
}
