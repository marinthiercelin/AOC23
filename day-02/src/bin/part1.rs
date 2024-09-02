use std::env;

use day_02::part1;

fn main() {
    let input = utils::read_input_file(env::args());
    let result = part1::run(&input);
    println!("Result: {result}");
}
