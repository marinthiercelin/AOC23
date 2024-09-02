use std::env;

use day_02::part2;

fn main() {
    let input = utils::read_input_file(env::args());
    let result = part2::run(&input);
    println!("Result: {result}");
}
