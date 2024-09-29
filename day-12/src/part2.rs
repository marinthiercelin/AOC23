use std::{cell::RefCell, collections::HashMap};

use crate::common::{SpringState, SpringsRow};
use SpringState::{Working, Broken};

pub fn run(input: &str) -> String {
    let sum: u64 = input.lines().map(SpringsRow::parse_with_duplication).map(|row| row.count_valid_states_recursive()).sum();
    sum.to_string()
}

impl SpringsRow {
    fn parse_with_duplication(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let states: Vec<Option<SpringState>> = parts.next().unwrap().chars().map(|c|{
            match c {
                '.' => Some(Working),
                '#' => Some(Broken),
                '?' => None, 
                _ => panic!("Invalid character: {c}")
            }
        }).collect();
        let broken_groups: Vec<u32> = parts.next().unwrap().split(",").map(|size| size.parse().unwrap()).collect();
        let mut duplicated_states = states.clone();
        let mut duplicated_broken_groups = broken_groups.clone();
        for _ in 1..=4 {
            duplicated_states.push(None);
            duplicated_states.extend_from_slice(&states);
            duplicated_broken_groups.extend_from_slice(&broken_groups);
        }
        Self{ states: duplicated_states, broken_groups: duplicated_broken_groups}
    }

    fn count_valid_states_recursive(&self) -> u64 {
        count_valid_states_recursive(&self.states, &self.broken_groups, &mut HashMap::new())
    }
}

impl SpringState {

    fn to_char(&self) -> char {
        match self {
            Working => '.',
            Broken => '#',
        }
    }
    
}

fn to_string(states: &[Option<SpringState>], groups: &[u32]) -> String {
    let state_string: String = states.iter().map(|state| state.map_or('?', |state| state.to_char())).collect();
    let group_string: String = groups.iter().map(|g| g.to_string()).collect::<Vec<String>>().join(",");
    format!("{state_string} {group_string}")
}


fn count_valid_states_recursive(states: &[Option<SpringState>], groups: &[u32], saved_results: &mut HashMap<String, u64>) -> u64 {

    let key = to_string(states, groups);

    if saved_results.contains_key(&key) {
        return *saved_results.get(&key).unwrap();
    }

    let saved_results = RefCell::new(saved_results);

    let body = || {
        if groups.is_empty() {
            return if states.contains(&Some(Broken)) {
                0
            } else {
                1
            };
        }
    
        if states.is_empty() {
            return 0;
        }
    
        let first_state = states[0];
    
        let first_group = groups[0] as usize;

        let broken_logic = || {
            if states.len() < first_group {
                return 0;
            }
            let potential_group = &states[..first_group];
            if potential_group.contains(&Some(Working)) {
                return 0;
            }
            if states.len() == first_group {
                return if groups.len() == 1 {
                    1
                } else {
                    0
                };
            }
            let state_after_group = states[first_group];
            if state_after_group.is_none() || state_after_group.unwrap() == Working {
                return count_valid_states_recursive(&states[first_group+1..], &groups[1..], &mut saved_results.borrow_mut());
            }
            return 0;
        };
    
        let working_logic = || {
            count_valid_states_recursive(&states[1..], groups, &mut saved_results.borrow_mut())
        };
    
        match first_state {
            Some(Working) => working_logic(),
            Some(Broken) => broken_logic(),
            None => working_logic() + broken_logic(),
        }
    };

    let result = body();
    saved_results.borrow_mut().insert(key, result);
    return result;
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
        let expected_output = "525152"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_parse_with_duplication() {
        let input = ".# 1"; // Add your test input here
        let expected_output = SpringsRow::parse(".#?.#?.#?.#?.# 1,1,1,1,1"); // Add the expected output here
        assert_eq!(SpringsRow::parse_with_duplication(input), expected_output);
        let input = "???.### 1,1,3"; // Add your test input here
        let expected_output = SpringsRow::parse("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3"); // Add the expected output here
        assert_eq!(SpringsRow::parse_with_duplication(input), expected_output);
    }

    #[test]
    fn test_starts_with_empty() {
        assert!(vec![1, 2, 3].starts_with(&vec![]))
    }

    #[test]
    fn test_count_valid_states_recursive() {
        let input = "???.### 1,1,3";
        let row = SpringsRow::parse_with_duplication(input);
        assert_eq!(row.count_valid_states_recursive(), 1)
        
    }

    // Add more tests here
}