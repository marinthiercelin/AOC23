pub fn run(input: &str) -> String {
    let patterns = input.split("\n\n");
    let patterns = patterns.map(Pattern::parse);
    let mut sum = 0;
    for pattern in patterns {
        let row_symmetry = find_symmetry(&pattern.encode_rows());
        if let Some(value) = row_symmetry {
            sum += value * 100;
        } else {
            let column_symmetry = find_symmetry(&&pattern.encode_columns());
            sum += column_symmetry.unwrap();
        }
    }
    sum.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SoilType {
    Rock,
    Ash,
}

struct Pattern {
    values: Vec<Vec<SoilType>>
}

impl SoilType {
    fn parse(val: char) -> Self {
        match val {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Invalid value {val}")
        }
    }
}

impl Pattern {
    fn parse(pattern: &str) -> Self {
        let values = pattern.lines().map(|line| line.chars().map(SoilType::parse).collect()).collect();
        Self{ values }
    }
}

impl Pattern {
    fn encode_rows(&self) -> Vec<u32> {
        let rows = self.values.iter();
        rows.map(|row| encode_as_int(row)).collect()
    }

    fn encode_columns(&self) -> Vec<u32> {
        let columns = (0..self.values[0].len()).into_iter().map(|column_index|self.values.iter().map(|row| row[column_index]).collect::<Vec<SoilType>>());
        columns.map(|column| encode_as_int(&column)).collect()
    }
}

fn encode_as_int(values: &[SoilType]) -> u32 {
    values.into_iter().enumerate().map(|(index, value)| {
        match value {
            SoilType::Ash => 0,
            SoilType::Rock => 1 << index,
        }
    }).sum()
}

fn find_symmetry(values: &[u32]) -> Option<usize> {
    // println!("{:?}", values);
    let values_reversed : Vec<u32> = Vec::from_iter(values.iter().rev().map(|&val|val));
    // println!("{:?}", values_reversed);
    for separation in 1..values.len() {
        let left_reversed = &values_reversed[values.len() - separation..];
        let right = &values[separation..];
        // println!("{separation} {:?} {:?}", left_reversed, right);
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
    fn test_encode_columns() {
        let pattern = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let pattern = Pattern::parse(pattern);
        let encoded = pattern.encode_columns();
        assert_eq!(vec![77, 12, 115, 33, 82, 82, 33, 115, 12], encoded);
    }

    #[test]
    fn test_find_symmetry() {
        let input = vec![77, 12, 115, 33, 82, 82, 33, 115, 12];
        let output = find_symmetry(&input);
        assert_eq!(Some(5), output)
    }

    // Add more tests here
}
