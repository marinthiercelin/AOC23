
use crate::common::{Network, Pulse, HIGH};

pub fn run(input: &str) -> String {
    let mut network = Network::parse(input);
    let mut high_count = 0;
    let mut low_count = 0;
    let mut accounting = |pulse: &Pulse| {
        if pulse.level == HIGH {
            high_count += 1;
        } else {
            low_count += 1;
        }
    };
    (0..1000).for_each(|_|{
        network.press_button(&mut accounting)
    });
    let score = high_count * low_count;
    return score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let expected_output = "32000000";
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_run_2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let expected_output = "11687500";
        assert_eq!(run(input), expected_output);
    }
}
