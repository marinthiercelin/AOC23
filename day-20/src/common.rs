use std::{collections::{HashMap, VecDeque}, ops::Not};

pub type PulseLevel = bool;
pub const HIGH: PulseLevel = true;
pub const LOW: PulseLevel = false;

pub struct Pulse {
    pub source: String,
    pub destination: String,
    pub level: PulseLevel
}

type FlipFlopState = bool;
const ON: FlipFlopState = true;
const OFF: FlipFlopState = false;

#[derive(Debug, PartialEq, Eq)]
pub enum ModuleState {
    FlipFlop(FlipFlopState),
    Conjonction(HashMap<String, PulseLevel>),
    Broadcast,
}

impl ModuleState {
    fn process_pulse(&mut self, pulse: &Pulse) -> Option<PulseLevel> {
        match self {
            ModuleState::FlipFlop(state) => {
                if pulse.level == HIGH {
                    None
                } else {
                    *state = state.not();
                    if *state == ON {
                        Some(HIGH)
                    } else {
                        Some(LOW)
                    }
                }
            },
            ModuleState::Conjonction(last_inputs) => {
                assert!(last_inputs.contains_key(&pulse.source));
                last_inputs.insert(pulse.source.clone(), pulse.level);
                if last_inputs.values().all(|&input| input == HIGH) {
                    Some(LOW)
                } else {
                    Some(HIGH)
                }
            },
            ModuleState::Broadcast => Some(pulse.level),
        }
    }

    fn reset(&mut self) {
        match self {
            ModuleState::FlipFlop(state) => *state = OFF,
            ModuleState::Conjonction(input_states) => input_states.values_mut().for_each(|val| *val = LOW),
            ModuleState::Broadcast => {},
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Module {
    pub name: String,
    pub state: ModuleState,
    pub destinations: Vec<String>
}

impl Module {
    pub fn process_pulse(&mut self, pulse: &Pulse) -> Option<Vec<Pulse>> {
        assert_eq!(self.name, pulse.destination);
        let level = self.state.process_pulse(pulse);
        return level.map(|level|{
            self.destinations
            .iter().
            map(|dest| Pulse{level, destination: dest.clone(), source: self.name.clone() })
            .collect()
        });
    }

    fn reset(&mut self) {
        self.state.reset();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Network {
    pub modules: HashMap<String, Module>,
}

impl Network {
    pub fn parse(input: &str) -> Network {
        let mut modules = HashMap::new();
        let mut module_inputs : HashMap<String, Vec<String>> = HashMap::new();
        input.lines().for_each(|line|{
            let mut split = line.split(" -> ");
            let name = split.next().unwrap();
            let destinations = split.next().unwrap();
            let destinations: Vec<String> = destinations.split(", ").map(|dest| dest.to_string() ).collect();
            let (name, state) = if name == "broadcaster" {
                (name, ModuleState::Broadcast)
            } else if name.starts_with("%") {
                (&name[1..], ModuleState::FlipFlop(OFF))
            } else if name.starts_with("&") {
                (&name[1..], ModuleState::Conjonction(HashMap::new()))
            } else {
                panic!("Invalid name: {name}");
            };
            destinations.iter().for_each(|dest|{
                if !module_inputs.contains_key(dest) {
                    module_inputs.insert(dest.clone(), vec![name.to_string()]);
                } else {
                    module_inputs.get_mut(dest).unwrap().push(name.to_string());
                } 
            });
            let module = Module{ destinations, name: name.to_string(), state};
            modules.insert(name.to_string(), module);
        });
        for (name, module) in &mut modules {
            if let ModuleState::Conjonction(last_inputs) = &mut module.state {
                let inputs = module_inputs.remove(name).expect("Conjonction without inputs");
                inputs.into_iter().for_each(|input|{
                    last_inputs.insert(input, LOW);
                });
            }
        }
        Network { modules }
    }

    pub fn press_button(&mut self, mut accounting: impl FnMut (&Pulse) -> ()) {
        let mut pending_pulses = VecDeque::new();
        let button_pulse = Pulse{destination: "broadcaster".to_string(), source : "button".to_string(), level: LOW};
        pending_pulses.push_back(button_pulse);
        while let Some(pulse) = pending_pulses.pop_front() {
            accounting(&pulse);
            if let Some(destination_module) = self.modules.get_mut(&pulse.destination) {
                if let Some(new_pulses) = destination_module.process_pulse(&pulse){
                    new_pulses.into_iter().for_each(|pulse| pending_pulses.push_back(pulse));
                }   
            }
        }
    }

    pub fn reset(&mut self) {
        self.modules.values_mut().for_each(|module| module.reset());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let network = Network::parse(input);
        let expected_network = Network {
            modules: [
                ("broadcaster".to_string(), Module {
                    name: "broadcaster".to_string(),
                    state: ModuleState::Broadcast,
                    destinations: ["a", "b", "c"].into_iter().map(ToString::to_string).collect()
                }),
                ("a".to_string(), Module {
                    name: "a".to_string(),
                    state: ModuleState::FlipFlop(OFF),
                    destinations: vec!["b"].into_iter().map(ToString::to_string).collect()
                }),
                ("b".to_string(), Module {
                    name: "b".to_string(),
                    state: ModuleState::FlipFlop(OFF),
                    destinations: vec!["c"].into_iter().map(ToString::to_string).collect()
                }),
                ("c".to_string(), Module {
                    name: "c".to_string(),
                    state: ModuleState::FlipFlop(OFF),
                    destinations: vec!["inv"].into_iter().map(ToString::to_string).collect()
                }),
                ("inv".to_string(), Module {
                    name: "inv".to_string(),
                    state: ModuleState::Conjonction([("c".to_string(), LOW)].into()),
                    destinations: vec!["a"].into_iter().map(ToString::to_string).collect()
                })
            ].into_iter().collect()
        };
        assert_eq!(network, expected_network)
    }
}