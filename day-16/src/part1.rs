use crate::common::{BeamHead, Contraption, Direction};

pub fn run(input: &str) -> String {
    let contraption = Contraption::parse(input);
    let original_beam = BeamHead{position: (0,0), direction: Direction::Right};
    let energized = contraption.energize(original_beam);
    energized.len().to_string()
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let expected_output = "46";
        assert_eq!(run(input), expected_output);
    }
}
