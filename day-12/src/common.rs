use SpringState::{Working, Broken};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SpringState {
    Working,
    Broken
}

#[derive(PartialEq, Eq, Debug)]
pub struct SpringsRow {
    pub states: Vec<Option<SpringState>>,
    pub broken_groups: Vec<u32>,
}

impl SpringsRow {
    pub fn parse(line: &str) -> Self {
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
}

pub fn broken_groups(states: &[SpringState]) -> Vec<u32> {
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

    use SpringState::{Working, Broken};

    #[test]
    fn test_broken_groups() {
        let input = vec![Working, Broken, Broken, Working, Working, Broken, Working, Broken, Broken];
        let expected_output = vec![2, 1, 2];
        let output = broken_groups(&input);
        assert_eq!(output, expected_output)
    }
}