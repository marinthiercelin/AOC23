use crate::common::Race;

pub fn run(input: &str) -> String {
    let race = parse(input);
    race.solve().to_string()
}

fn parse(input: &str) -> Race {
    let mut lines = input.lines();
    let time = lines.next().unwrap().split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse().unwrap();
    let distance_to_beat = lines.next().unwrap().split_whitespace().skip(1).collect::<Vec<&str>>().join("").parse().unwrap();
    Race { time, distance_to_beat }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn test_run() {
        let input = "Time:      7  15   30
Distance:  9  40  200"; // Add your test input here
        let expected_output = "71503"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_parse() {
        parse(&fs::read_to_string("input.txt").unwrap());
    }

    // Add more tests here
}