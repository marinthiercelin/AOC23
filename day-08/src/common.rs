use std::collections::HashMap;

pub type NodeKey = String;

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<NodeKey, Node>,
}

pub enum Instruction {
    Left,
    Right
}

#[derive(Debug)]
pub struct Node  {
    pub key: NodeKey,
    pub left: NodeKey,
    pub right: NodeKey,
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

pub fn parse_instructions_and_graphs(input: &str) -> (Vec<Instruction>, Graph) {
    let mut lines = input.lines();
    let instructions = parse_instructions(lines.next().unwrap());
    lines.next(); // Skip the empty line
    let nodes = lines.map(|l| parse_node(l)).map(|node| (node.key.clone(), node)).collect();
    (instructions, Graph { nodes })
}