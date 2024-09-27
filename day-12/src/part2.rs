use std::{sync::mpsc::channel, thread};

use crate::common::{broken_groups, SpringState, SpringsRow};
use SpringState::{Working, Broken};

pub fn run(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut sum = 0;
    let mut done = 0;
    let total = lines.len();
    for line in lines {
        println!("Starting row parsing");
        let row = SpringsRow::parse_with_duplication(line);
        println!("Starting row counting");
        let count = row.count_valid_states_recursive();
        sum += count;
        done += 1;
        println!("Progress {done}/{total}");
    }
    sum.to_string()
}

// pub fn run_parallel(input: &str) -> String {
    

//     let lines: Vec<&str> = input.lines().collect();

//     let num_rows = lines.len();

//     println!("number of rows : {}", num_rows );

//     let (sender, receiver) = channel(); 

//     lines.into_iter().for_each(|line|{
//         let sender = sender.clone();
//         let line = line.to_string();
//         thread::spawn(move || {
//             let row = SpringsRow::parse_with_duplication(&line);
//             sender.send(row.count_valid_states_recursive()).unwrap();
//         });
//     });

//     let mut received = 0;
//     let mut sum = 0;
//     while received != num_rows {
//         let count = receiver.recv().unwrap();
//         received += 1;
//         println!("Progress: {received}/{num_rows}");
//         sum += count;
//     }
//     return sum.to_string();
// }

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
}

impl SpringsRow {
    fn count_valid_states_recursive(&self) -> u32 {
        self.explore_valid_states(&[])
    }

    fn is_coherent(&self, fixed: &[SpringState]) -> bool {
        let broken_groups = broken_groups(&fixed);
        if broken_groups.is_empty() {
            return true;
        }
        if broken_groups.len() > self.broken_groups.len() {
            return false;
        }
        let last_index = broken_groups.len() - 1;
        let prefix_is_coherent = self.broken_groups.starts_with(&broken_groups[..last_index]);
        let last_is_coherent = match fixed.last().unwrap() {
            Working => {
                broken_groups[last_index] == self.broken_groups[last_index]
            }
            Broken => {
                // the last group potentially includes the next unknowns
                broken_groups[last_index] <= self.broken_groups[last_index]
            }
        };
        prefix_is_coherent && last_is_coherent
    }

    fn explore_valid_states(&self, fixed: &[SpringState]) -> u32 {

        let to_fix = &self.states[fixed.len()..];

        let mut fixed = Vec::from(dbg!(fixed));
    
        if !to_fix.is_empty() {
            // skip to the next unknown state
            for state in to_fix {
                if state.is_none() {
                    break;
                }
                fixed.push(state.unwrap());
            }
        }
    
        let to_fix = &self.states[fixed.len()..];
        
        // Check if we are done
        if to_fix.is_empty() {
            return if broken_groups(&fixed) == *self.broken_groups {
                // Reached a solution
                1 
            } else { 
                0 
            };
        }
        
        // First set the unknown state to working
        fixed.push(Working);
        let working_branch_count = if !self.is_coherent(&fixed) {
            0
        } else {
            self.explore_valid_states(&fixed)
        };
        fixed.pop();
    
        // Then set the unknown state to broken
        fixed.push(Broken);
        let broken_branch_count = if !self.is_coherent(&fixed) {
            0
        } else {
            self.explore_valid_states(&fixed)
        };
        fixed.pop();
    
    
        return working_branch_count + broken_branch_count;
    }
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

    #[test]
    fn test_is_coherent() {
        let input = "???.### 1,1,3";
        let row = SpringsRow::parse_with_duplication(input);
        let solution: Vec<SpringState> = vec![vec![Broken, Working, Broken, Working, Broken, Broken, Broken, Working];5].into_iter().flatten().collect();
        for i in 0..solution.len() -1 {
            assert!(row.is_coherent(&solution[..i]))
        }
        
    }

    // Add more tests here
}