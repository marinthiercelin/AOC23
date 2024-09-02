pub fn run(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| find_value(line))
        .map(|number| number.parse::<u32>().unwrap())
        .sum();
    result.to_string()
}

fn find_value(line: &str) -> String {
    let mut numbers = line.chars().filter(|c| c.is_numeric());
    let first = numbers.next().unwrap();
    let last = numbers.last().unwrap_or(first);
    return vec![first, last].into_iter().collect();
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn run_test_input() {
        let expected = "142";
        let actual = run(&fs::read_to_string("test_part1.txt").unwrap());
        assert_eq!(expected, actual)
    }
}
