use core::net;
use std::{collections::VecDeque, fmt::format};

use num::integer;

use crate::common::{Module, Network, Pulse, HIGH, LOW};

pub fn run(input: &str) -> String {
    let mut network = Network::parse(input);
    let rx_ancestors = ["vk", "ks", "dl", "pm"];
    let mut cycles = vec![];
    for ancestor in rx_ancestors {
        let mut count_buttons = 0;
        loop {
            let mut count = 0;
            let mut accounting = |pulse: &Pulse| {
                if pulse.source == ancestor && pulse.level == HIGH {
                    count += 1;
                }
            };
            network.press_button(&mut accounting);
            count_buttons += 1;
            if count == 1 {
                break;
            }
        }
        println!("{ancestor} emmited HIGH after {count_buttons}");
        cycles.push(count_buttons);
        network.reset();
    }
    cycles.into_iter().fold(1, |acc, cycle|{
        integer::lcm(acc, cycle as u64)
    }).to_string()
}

impl Network {
    fn print_paths(&self, target: &str) {
        self.print_paths_rec("broadcaster", &[], target);   
    }



    fn print_paths_rec(&self, node: &str, path_to_node: &[&str], target: &str) {
        if node == target {
            println!("{:?}", path_to_node);
            return;
        }
        
        if let Some(module) = self.modules.get(node) {
            let mut path = Vec::from(path_to_node);
            let display = module.display();
            if path_to_node.contains(&display.as_str()) {
                return;
            }
            path.push(&display);
            for dest in &module.destinations {
                self.print_paths_rec(dest, &path, target);
            }
        }
    }
}

impl Module {
    fn display(&self) -> String {
        match &self.state {
            crate::common::ModuleState::FlipFlop(_) => format!("%{}", self.name),
            crate::common::ModuleState::Conjonction(_) => format!("&{}", self.name),
            crate::common::ModuleState::Broadcast => self.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "";
        let expected_output = "";
        assert_eq!(run(input), expected_output);
    }

}

