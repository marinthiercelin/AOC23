
use crate::common::Race;

pub fn run(input: &str) -> String {
    let races = parse(input);
    let solutions = races.iter().map(|race| race.solve()).collect::<Vec<_>>();
    let margin : u32 = dbg!(solutions).iter().product();
    margin.to_string()
}


fn parse(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times = lines.next().unwrap().split_whitespace().skip(1).map(|s| s.parse().unwrap());
    let distances = lines.next().unwrap().split_whitespace().skip(1).map(|s| s.parse().unwrap());
    times.into_iter().zip(distances).map(|(time, distance_to_beat)| Race { time, distance_to_beat}).collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "Time:      7  15   30
Distance:  9  40  200"; // Add your test input here
        let expected_output = "288"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}
