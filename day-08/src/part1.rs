use std::collections::HashMap;

pub fn run(input: &str) -> String {
    let (instructions, graph) = parse_instructions_and_graphs(input);
    // let graph = dbg!(graph);
    let mut current_node = "AAA";
    let mut instruction_count = 0;
    let instructions_length = instructions.len();
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
type NodeKey = String;

#[derive(Debug)]
struct Graph {
    nodes: HashMap<NodeKey, Node>,
}

enum Instruction {
    Left,
    Right
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    line.chars().map(|c| match c {
        'L' => Instruction::Left,
        'R' => Instruction::Right,
        _ => panic!("Invalid instruction: {}", c),
    }).collect()
}

fn parse_node(line: &str) -> Node {
    let parts: Vec<&str> = line.split(" = ").collect();
    let key = parts[0].to_string();
    let children: Vec<&str> = parts[1].split(", ").collect();
    Node { key, left: children[0][1..].to_string(), right: children[1][..children[1].len() - 1].to_string() }
}

fn parse_instructions_and_graphs(input: &str) -> (Vec<Instruction>, Graph) {
    let mut lines = input.lines();
    let instructions = parse_instructions(lines.next().unwrap());
    lines.next(); // Skip the empty line
    let nodes = lines.map(|l| parse_node(l)).map(|node| (node.key.clone(), node)).collect();
    (instructions, Graph { nodes })
}

#[derive(Debug)]
struct Node  {
    key: NodeKey,
    left: NodeKey,
    right: NodeKey,
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
