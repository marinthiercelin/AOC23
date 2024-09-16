use crate::common::History;

pub fn run(input: &str) -> String {
    let histories = input.lines().map(History::parse).collect::<Vec<History>>();
    let extrapolated = histories.iter().map(|h| h.extrapolate_backward()).collect::<Vec<i32>>();
    extrapolated.iter().sum::<i32>().to_string()
}

impl History {
    fn extrapolate_backward(&self) -> i32 {
        let mut diffs = self.values.clone();
        let mut first_values = Vec::new();
        while !diffs.iter().all(|&d| d == 0) {
            first_values.push(diffs[0]);
            diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect();
        }
        first_values.reverse();
        let extrapolated = first_values
        .iter()
        .fold(0, |acc, &value| value - acc);
        extrapolated
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
        let expected_output = "2"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}
