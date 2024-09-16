use crate::common::History;

pub fn run(input: &str) -> String {
    let histories = input.lines().map(History::parse);
    let extrapolated = histories.map(|h| h.extrapolate());
    extrapolated.sum::<i32>().to_string()
}

impl History {
    fn extrapolate(&self) -> i32 {
        let mut diffs = self.values.clone();
        let mut last_values = Vec::new();
        while !diffs.iter().all(|&d| d == 0) {
            last_values.push(*diffs.last().unwrap());
            diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        }
        last_values.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"; // Add your test input here
        let expected_output = "114"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}
