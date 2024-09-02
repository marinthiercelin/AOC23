use crate::common;

pub fn run(input: &str) -> String {
    let (_, gears) = common::parse_and_analyze_engine(input);
    let total_ratio = gears.iter().map(|gear| gear.neighbors[0] * gear.neighbors[1]).sum::<u32>();
    total_ratio.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let expected = "467835";
        let result = run(input);
        assert_eq!(result, expected);
    }
}