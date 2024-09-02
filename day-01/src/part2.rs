use regex::Regex;

pub fn run(input: &str) -> String {
    let result : u32 = input
            .lines()
            .map(|line| find_value(line))
            .sum();
        result.to_string()
}

fn find_value(line: &str) -> u32 {
    let mut line_scanned = line;
    let re = Regex::new(r"([0-9]|zero|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let mut matches: Vec<&str> = Vec::new();
    loop {
        match re.find(line_scanned) {
            Some(match_item) => {
                matches.push(match_item.as_str());
                let skip_index = match_item.start() + 1;
                line_scanned = &line_scanned[skip_index..]
            },
            None => break,
        }
    }
    let mut matches_iter = matches.into_iter();
    let first = matches_iter.next().map(|x| parse_integer(x)).unwrap();
    let last = matches_iter.last().map(|x| parse_integer(x)).unwrap_or(first);
    return first*10 + last;
}

// a function can parse a digit from 0 to 9 or a string from "zero" to "nine"
fn parse_integer(input: &str) -> u32 {
    match input {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => panic!("Invalid input")
    }
}


mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn run_test_input() {
        let expected = "281";
        let actual = run( &fs::read_to_string("test_part2.txt").unwrap());
        assert_eq!(expected, actual)
    }
}