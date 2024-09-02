use crate::common;

pub fn run(input: &str) -> String {
    let (part_numbers, _) = common::parse_and_analyze_engine(input);
    let result = part_numbers.iter().map(|n| n.value).sum::<u32>();
    result.to_string()
}



#[cfg(test)]
mod tests {

    use super::*;
    

    #[test]
    fn run_test_input() {
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
        let expected = "4361";
        let actual = run(input);
        assert_eq!(expected, actual)
    }
}