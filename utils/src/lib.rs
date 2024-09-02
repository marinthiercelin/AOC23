use std::fs;

pub fn read_input_file(mut args: impl Iterator<Item = String>) -> String {
    match args.nth(1) {
        Some(input_file_path) => fs::read_to_string(input_file_path).unwrap(),
        None => panic!("No input file path was provided"),
    }
}
