use crate::common::{Line, Pattern, SoilType};

pub fn run(input: &str) -> String {
    let patterns = input.split("\n\n");
    let patterns = patterns.map(Pattern::parse);
    let mut sum = 0;
    for pattern in patterns {
        sum += pattern.value(find_symmetry)
    }
    sum.to_string()
}

fn encode_as_int(values: &Line) -> u32 {
    values.into_iter().enumerate().map(|(index, value)| {
        match value {
            SoilType::Ash => 0,
            SoilType::Rock => 1 << index,
        }
    }).sum()
}

fn find_symmetry(values: &[Line]) -> Option<usize> {
    let values: Vec<u32> = values.iter().map(|line| encode_as_int(line)).collect();
    let values_reversed : Vec<u32> = Vec::from_iter(values.iter().rev().map(|&val|val));
    for separation in 1..values.len() {
        let left_reversed = &values_reversed[values.len() - separation..];
        let right = &values[separation..];
        let (longest, shortest) = if left_reversed.len() > right.len() {
            (left_reversed, right)
        } else {
            (right, left_reversed)
        };
        
        if longest.starts_with(shortest) {
            return Some(separation);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"; // Add your test input here
        let expected_output = "405"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_find_symmetry() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let pattern = Pattern::parse(&input);
        let output = find_symmetry(&pattern.columns());
        assert_eq!(Some(5), output)
    }

    // Add more tests here
}
