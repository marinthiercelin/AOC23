
use crate::common::{self, Instruction};

pub fn run(input: &str) -> String {
    let (instructions, graph) = common::parse_instructions_and_graphs(input);
    // let graph = dbg!(graph);
    let mut current_node = "AAA";
    let mut instruction_count = 0;
    let instructions_length = dbg!(instructions.len());

    while current_node != "ZZZ" {
        let instruction = &instructions[instruction_count % instructions_length];
        current_node = match instruction {
            Instruction::Left => &graph.nodes[current_node].left,
            Instruction::Right => &graph.nodes[current_node].right,
        };
        instruction_count += 1;
    }
    return instruction_count.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"; // Add your test input here
        let expected_output = "2"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_run_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"; // Add your test input here
        let expected_output = "6"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}
