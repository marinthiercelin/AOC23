use crate::common::{Line, Pattern};

pub fn run(input: &str) -> String {
    let patterns = input.split("\n\n");
    let patterns = patterns.map(Pattern::parse);
    let mut sum = 0;
    for pattern in patterns {
        sum += pattern.value(find_symmetry_with_smudge)
    }
    sum.to_string()
}

fn distance(line: &Line, other_line: &Line) -> usize {
    line.iter().zip(other_line).filter(|(a, b)| a != b).count()
}

fn find_symmetry_with_smudge(values: &[Line]) -> Option<usize> {
    let values = Vec::from_iter(values.iter());
    let values_reversed : Vec<&Line> = Vec::from_iter(values.iter().rev().map(|&x|x));
    for separation in 1..values.len() {
        let left_reversed = &values_reversed[values.len() - separation..];
        let right = &values[separation..];
        let (longest, shortest) = if left_reversed.len() > right.len() {
            (left_reversed, right)
        } else {
            (right, left_reversed)
        };
        if match_with_smudge(shortest, longest) {
            return Some(separation)
        }
    }
    None
}

fn match_with_smudge(shortest: &[&Line], longest: &[&Line]) -> bool {
    let mut seen_smudge = false;
    for (index, &shortest_val) in shortest.iter().enumerate() {
        let longest_val = longest[index];
        match distance(&shortest_val, longest_val) {
            0 => continue,
            1 => {
                if seen_smudge {
                    return false;
                }
                seen_smudge = true;
            },
            _ => { return false; }
        }
    }
    return seen_smudge;
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
        let expected_output = "400"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}