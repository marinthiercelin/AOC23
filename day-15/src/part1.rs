use crate::common;

pub fn run(input: &str) -> String {
    let input = input.replace("\n", "");
    let instructions = input.split(",");
    let result = instructions.map(|instruction| common::hash(instruction)).sum::<u32>();
    result.to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"; // Add your test input here
        let expected_output = "1320"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}
