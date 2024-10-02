
use crate::common;

pub fn run(input: &str) -> String {
    let input = input.replace("\n", "");
    let instructions = input.split(",");
    let instructions = instructions.map(Instruction::parse);
    let mut box_series = BoxSeries::new();
    instructions.for_each(|instruction| box_series.follow_instruction(instruction));
    box_series.focusing_power().to_string()
}

struct Lense {
    label: String,
    focal_length: u8
}

struct Box {
    lenses: Vec<Lense>
}

struct BoxSeries {
    boxes: Vec<Box>
}

enum InstructionType {
    RemoveLens,
    AssignLens{focal_length: u8}
}

struct Instruction {
    label: String,
    instruction_type: InstructionType
}

impl Box {
    fn new() -> Self {
        return Self { lenses: Vec::new()}
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        match instruction.instruction_type {
            InstructionType::AssignLens { focal_length } => {
                let existing_lense = self.lenses.iter_mut().find(|lense| lense.label == instruction.label);
                if let Some(existing_lense) = existing_lense {
                    existing_lense.focal_length = focal_length;
                } else {
                    self.lenses.push(Lense { label: instruction.label, focal_length });
                }
            }
            InstructionType::RemoveLens => {
                let lense = self.lenses.iter_mut().enumerate().find(|(_,lense)| lense.label == instruction.label);
                if let Some((lense_index, _)) = lense {
                    self.lenses.remove(lense_index);
                }
            }
        }
    }

    fn focusing_power(&self) -> u32 {
        self.lenses
        .iter()
        .enumerate()
        .map(|(index, lense)| (index+1) as u32 * lense.focal_length as u32)
        .sum()
    }

}

impl BoxSeries {
    fn new() -> Self {
        let boxes = (0..=255).map(|_| Box::new()).collect();
        return Self { boxes };
    }

    fn follow_instruction(&mut self, instruction: Instruction) {
        let key = common::hash(&instruction.label);
        self.boxes[key as usize].follow_instruction(instruction)
    }

    fn focusing_power(&self) -> u32{
        self.boxes
        .iter()
        .enumerate()
        .map(|(index, lense_box)| (index+1) as u32 * lense_box.focusing_power())
        .sum()
    }
}


impl Instruction {
    fn parse(input: &str) -> Self {
        assert!(!input.is_empty());
        let len = input.len();
        if input.ends_with("-"){
            let label =input[..len - 1].to_string();
            Self { label , instruction_type: InstructionType::RemoveLens}
        } else {
            let mut parts = input.split("=");
            let label = parts.next().unwrap().to_string();
            let focal_length = parts.next().unwrap().parse().unwrap();
            let instruction_type = InstructionType::AssignLens { focal_length };
            Self {label, instruction_type}
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"; // Add your test input here
        let expected_output = "145"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}