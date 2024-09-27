use std::result;

use SpringState::{Working, Broken};

pub fn run(input: &str) -> String {
    let rows = input.lines().map(SpringsRow::parse);
    let counts = rows.map(|row| row.count_valid_states());
    let result: usize = counts.sum();
    result.to_string()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpringState {
    Working,
    Broken
}

struct SpringsRow {
    states: Vec<Option<SpringState>>,
    broken_groups: Vec<u32>,
}

impl SpringsRow {
    fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let states = parts.next().unwrap().chars().map(|c|{
            match c {
                '.' => Some(Working),
                '#' => Some(Broken),
                '?' => None, 
                _ => panic!("Invalid character: {c}")
            }
        }).collect();
        let broken_groups = parts.next().unwrap().split(",").map(|size| size.parse().unwrap()).collect();
        Self{ states, broken_groups}
    }

    fn all_possible_states(&self) -> StatesIterator {
        StatesIterator::new(&self)
    }

    fn count_valid_states(&self) -> usize {
        self.all_possible_states().filter(|states| broken_groups(states) == self.broken_groups).count()
    }
}

struct StatesIterator<'a> {
    row: &'a SpringsRow,
    count: u32,
    number_of_unknows: u32
}

impl<'a> StatesIterator<'a> {
    fn new(row: &'a SpringsRow) -> Self {
        let number_of_unknows = row.states.iter().filter(|state| state.is_none() ).count() as u32;
        Self { row, count: 0, number_of_unknows }
    }
}

impl Iterator for StatesIterator<'_> {
    type Item = Vec<SpringState>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == (1 << self.number_of_unknows) {
            // The bitmap reached the end;
            return None
        }
        let mut states = Vec::with_capacity(self.row.states.len());
        let mut unknown_index = 0usize;
        for &state in &self.row.states {
            if let Some(value) = state {
                states.push(value);
            } else {
                // count acts as a bitmap for unknown states
                let value = if (self.count >> unknown_index & 1) == 1 {
                    Working
                } else { 
                    Broken 
                };
                unknown_index += 1;
                states.push(value);
            };
        }
        self.count += 1; // Update the bitmap for the next state;
        Some(states)
    }
    
}

fn broken_groups(states: &Vec<SpringState>) -> Vec<u32> {
    let (mut groups, last_count) = states.iter().fold((Vec::new(), 0), |(mut groups, current_count), state|{
        match state {
            Working => {
                if current_count > 0 {
                    groups.push(current_count);
                }
                (groups, 0)
            },
            Broken => (groups, current_count + 1),
        }
    });
    if last_count > 0 {
        groups.push(last_count);
    }
    groups
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"; // Add your test input here
        let expected_output = "21"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_broken_groups() {
        let input = vec![Working, Broken, Broken, Working, Working, Broken, Working, Broken, Broken];
        let expected_output = vec![2, 1, 2];
        let output = broken_groups(&input);
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_count_valid_states() {
        let input = "???.### 1,1,3";
        let row = SpringsRow::parse(input);
        let expected_output = 1;
        let output = row.count_valid_states();
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_count_valid_states_2() {
        let input = ".??..??...?##. 1,1,3";
        let row = SpringsRow::parse(input);
        let expected_output = 4;
        let output = row.count_valid_states();
        assert_eq!(output, expected_output)
    }

    #[test]
    fn test_count_valid_states_3() {
        let input = "?###???????? 3,2,1";
        let row = SpringsRow::parse(input);
        let expected_output = 10;
        let output = row.count_valid_states();
        assert_eq!(output, expected_output)
    }

    // Add more tests here
}
