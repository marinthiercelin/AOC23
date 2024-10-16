
use crate::common::{self, Category, RuleCondition, RuleDestination, System, ACCEPT, REJECT};

pub fn run(input: &str) -> String {
    let (system, _) = common::parse_input(input);
    let solutions = system.count_accepted_combinations();
    solutions.to_string()
}

#[derive(Debug, Clone)]
struct Bounds {
    low: [u32;4],
    high: [u32; 4]
}

impl Bounds {
    fn new() -> Self {
        Self { low: [0;4], high: [4001;4] }
    }

    fn apply(mut self, condition: &RuleCondition) -> Self {
        let index = Self::category_index(&condition.category);
        if condition.lower_than_bound && self.high[index] > condition.bound{
            self.high[index] = condition.bound
        }
        if !condition.lower_than_bound && self.low[index] < condition.bound {
            self.low[index] = condition.bound
        }
        self
    }

    fn apply_not(mut self, condition: &RuleCondition) -> Self {
        let index = Self::category_index(&condition.category);
        // condition: cat < bound becomes cat >= bound or cat > bound - 1
        if condition.lower_than_bound && self.low[index] < condition.bound + 1 {
            self.low[index] = if condition.bound > 0 {
                condition.bound - 1
            } else {
                0
            }
        }
        // condition: cat > bound becomes cat <= bound or cat < bound + 1
        if !condition.lower_than_bound && self.high[index] > condition.bound + 1{
            self.high[index] = condition.bound + 1
        }
        self
    }

    fn category_index(category: &Category) -> usize {
        match category {
            Category::X => 0,
            Category::M => 1,
            Category::A => 2,
            Category::S => 3,
        }
    }

    fn count_solutions(&self) -> u64 {
        self.low.iter().zip(self.high.iter()).map(|(&low, &high)|{
            let cat_solutions = if low < high {
                high - low - 1
            } else {
                0
            };
            cat_solutions as u64
        }).product()
    }
}

impl System {
    fn count_accepted_combinations(&self) -> u64 {
        self.depth_first_search(&[], Bounds::new(), "in")
    }

    fn depth_first_search(&self, workflow_path: &[&str], bounds: Bounds, current_workflow_name: &str) -> u64 {
        if workflow_path.contains(&current_workflow_name) {
            return 0 // prevent cycles
        }
        let current_worflow = self.workflows.get(current_workflow_name).expect("Unknown workflow");
        let mut total_solutions = 0;
        let mut new_workflow_path = Vec::from(workflow_path);
        new_workflow_path.push(current_workflow_name);
        let mut bounds = bounds;
        for rule in &current_worflow.rules {
            let mut new_bounds = bounds.clone();
            if let Some(condition) = &rule.condition {
                new_bounds = new_bounds.apply(condition)
            }
            let rule_solutions = match &rule.destination {
                RuleDestination::Workflow { name: rule_destination } => {
                    self.depth_first_search(&new_workflow_path, new_bounds, &rule_destination)
                },
                RuleDestination::Decision(ACCEPT) => {
                    // Reached accept leave, count solutions
                    new_bounds.count_solutions()
                },
                RuleDestination::Decision(REJECT) => 0
            };
            total_solutions += rule_solutions;
            if let Some(condition) = &rule.condition {
                // moving on to the next rule is like applying the reverse of the rule
                bounds = bounds.apply_not(condition)
            }
        }
        total_solutions
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
        let expected_output = "167409079868000";
        assert_eq!(run(input), expected_output);
    }

}