use std::collections::HashMap;
use lazy_static::lazy_static;

use regex::Regex;

pub fn run(input: &str) -> String {
    let (system, parts) = parse_input(input);
    let accepted_parts : Vec<&Part> = parts.iter().filter(|part| system.check_part(part)).collect();
    let total: u32 = accepted_parts.iter().map(|part| part.total_rating()).sum();
    return total.to_string()
}

fn parse_input(input: &str) -> (System, Vec<Part>) {
    let split : Vec<&str> = input.split("\n\n").collect();
    assert_eq!(split.len(), 2);
    let system = System::parse(split[0]);
    let parts = split[1].lines().map(Part::parse).collect();
    (system, parts)
}

struct System {
    workflows: HashMap<String, Workflow>
}

impl System {
    fn parse(input: &str) -> Self {
        let workflows = input.lines().map(Workflow::parse).map(|w|(w.name.clone(), w)).collect();
        Self { workflows }
    }

    fn check_part(&self, part: &Part) -> Decision {
        let mut workflow = self.workflows.get("in").expect("No in workflow");
        loop {
            let destination: &RuleDestination = workflow.apply_to(part);
            if let &RuleDestination::Decision(decision) = destination {
                return decision
            }
            let name = destination.name();
            workflow = self.workflows.get(name).expect("Couldn't find destination workflow")
        }
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

    fn apply_to(&self, part: &Part) -> &RuleDestination {
        self.rules.iter().find_map(|rule| rule.apply_to(part)).expect("No rule are matching")
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

    fn apply_to(&self, part: &Part) -> Option<&RuleDestination> {
        if self.condition.as_ref().is_some_and(|condition| !condition.is_matching(part) ) {
            return None
        }
        Some(&self.destination)
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

    fn is_matching(&self, part: &Part) -> bool {
        let value = part.get_value(&self.category);
        if self.lower && value < self.bound {
            return true;
        }
        if !self.lower && value > self.bound {
            return true;
        }
        return false;
    }
}

type Decision = bool;
const ACCEPT: bool = true;
const REJECT: bool = false;

#[derive(Debug, Clone)]
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

    fn name(&self) -> &String {
        match self {
            RuleDestination::Workflow { name } => name,
            _ => panic!("Destination is not a workflow")
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

    fn get_value(&self, category: &Category) -> u32 {
        match  category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }

    fn total_rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
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
        let expected_output = "19114";
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
