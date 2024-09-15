use std::collections::HashSet;

use crate::common::{self, Instruction, NodeKey};
use num::integer;

pub fn run(input: &str) -> String {
    let (instructions, graph) = common::parse_instructions_and_graphs(input);
    let starting_nodes = graph.nodes.keys().filter(|key| key.ends_with("A")).collect::<Vec<&String>>();
    let periods: Vec<usize> = starting_nodes.iter().map(|starting_node| find_ending_state_period(starting_node, &instructions, &graph)).collect();
    let result = periods.iter().fold(1, |acc, period| integer::lcm(acc, *period));
    return result.to_string();
}

#[derive(Debug, Eq, Hash)]
struct EndingState {
    instructions_length: usize,
    instruction_index: usize,
    node: NodeKey,
}

impl PartialEq for EndingState {
    fn eq(&self, other: &Self) -> bool {
        if self.node != other.node {
            return false;
        }
        let diff = self.instruction_index as isize - other.instruction_index as isize;
        return diff.abs() as usize % self.instructions_length == 0;
    }
}

fn find_ending_state_period(starting_node: & NodeKey, instructions: &Vec<Instruction>, graph: & common::Graph) -> usize {
    let mut current_node = starting_node;
    let mut instruction_index = 0;
    let instructions_length = instructions.len();
    let mut ending_states = HashSet::new();
    loop {
        let instruction = &instructions[instruction_index % instructions_length];
        current_node = match instruction {
            Instruction::Left => &graph.nodes[current_node].left,
            Instruction::Right => &graph.nodes[current_node].right,
        };
        if current_node.ends_with("Z") {
            let ending_state = EndingState {
                instructions_length,
                instruction_index,
                node: current_node.clone(),
            };
            if ending_states.contains(&ending_state) {
                break;
            }
            ending_states.insert(ending_state);
        }
        instruction_index += 1;
    }
    let mut ending_states = ending_states.into_iter().collect::<Vec<EndingState>>();
    ending_states.sort_by_key(|ending_state| ending_state.instruction_index);
    ending_states[1].instruction_index - ending_states[0].instruction_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"; // Add your test input here
        let expected_output = "6"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}