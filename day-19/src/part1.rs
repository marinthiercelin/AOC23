use std::collections::HashMap;
use lazy_static::lazy_static;

use regex::Regex;

pub fn run(input: &str) -> String {
    todo!("Implement");
}

struct System {
    workflows: HashMap<String, Workflow>
}

impl System {
    fn parse(input: &str) -> Self {
        let workflows = input.lines().map(Workflow::parse).map(|w|(w.name.clone(), w)).collect();
        Self { workflows }
    }
}

struct Workflow {
    name: String,
    rules: Vec<Rule>
}

lazy_static! {
    static ref worflow_regex: Regex = Regex::new(r"(?<name>[a-z]+)\{(?<rules>.*)\}").unwrap();
}

impl Workflow {
    fn parse(line: &str) -> Self {
        let captures = worflow_regex.captures(line).expect("Line didn't match workflow regex");
        let name = captures["name"].to_string();
        let rules = captures["rules"].to_string();
        let rules = rules.split(",").map(Rule::parse).collect();
        Self { name, rules }
    }
}

struct Rule {
    condition: Option<RuleCondition>,
    destination: RuleDestination
}

impl Rule {
    fn parse(rule: &str) -> Self {
        if rule.contains(":") {
            let split: Vec<&str> = rule.split(":").collect();
            assert!(split.len() == 2);
            let condition = Some(RuleCondition::parse(split[0]));
            let destination = RuleDestination::parse(split[1]);
            Self { condition, destination }
        } else {
            let destination = RuleDestination::parse(rule);
            Self { condition: None, destination }
        }
    }
}

struct RuleCondition {
    category: Category,
    lower: bool,
    bound: u32,
}

impl RuleCondition {
    fn parse(condition: &str) -> Self {
        if condition.contains("<") {
            let split: Vec<&str> = condition.split("<").collect();
            assert!(split.len() == 2);
            let category = Category::parse(split[0]);
            let bound = split[1].parse().unwrap();
            Self { category, lower: true, bound}
        } else if condition.contains(">") {
            let split: Vec<&str> = condition.split(">").collect();
            assert!(split.len() == 2);
            let category = Category::parse(split[0]);
            let bound = split[1].parse().unwrap();
            Self { category, lower: false, bound}
        } else {
            panic!("Invalid condition");
        }
    }
}

type Decision = bool;
const ACCEPT: bool = true;
const REJECT: bool = false;

enum RuleDestination {
    Workflow{name: String},
    Decision(Decision)
}

impl RuleDestination {
    fn parse(destination: &str) -> Self{
        match destination {
            "A" => Self::Decision(ACCEPT),
            "R" => Self::Decision(REJECT),
            _ => Self::Workflow { name: destination.to_string() }
        }
    }
}

enum Category {
    X, M, A, S
}

impl Category {
    fn parse(category: &str) -> Self {
        match category {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Invalid category {category}")
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Part {
    x: u32, m: u32, a: u32, s: u32
}

lazy_static! {
    static ref part_regex: Regex = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
}

impl Part {
    fn parse(line: &str) -> Self {
        let captures = part_regex.captures(line).expect("line should match regex");
        let x = captures["x"].parse().expect("x not an int");
        let m = captures["m"].parse().expect("m not an int");
        let a = captures["a"].parse().expect("a not an int");
        let s = captures["s"].parse().expect("s not an int");
        Self{ x, m, a, s }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let expected_output = "";
        assert_eq!(run(input), expected_output);
    }

    #[test]
    fn test_parse_part() {
        let input = "{x=2127,m=1623,a=2188,s=1013}";
        let expected = Part { x: 2127, m: 1623, a: 2188, s: 1013};
        let actual = Part::parse(input);
        assert_eq!(expected, actual)
    }
}
