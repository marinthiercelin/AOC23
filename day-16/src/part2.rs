use crate::common::{BeamHead, Contraption, Direction};

pub fn run(input: &str) -> String {
    let contraption = Contraption::parse(input);
    let energized = contraption.find_max_energize();
    energized.to_string()
}

impl Contraption {
    fn find_max_energize(&self) -> usize {
        let max_row = self.tiles.len();
        let max_column = self.tiles[0].len();
        let right_beams = (0..max_row).map(|row| BeamHead{position: (row, 0), direction: Direction::Right});
        let left_beams = (0..max_row).map(|row| BeamHead{position: (row, max_column - 1), direction: Direction::Left});
        let down_beams = (0..max_column).map(|column| BeamHead{ position: (0, column), direction: Direction::Down});
        let up_beams = (0..max_column).map(|column| BeamHead{ position: (0, column), direction: Direction::Up});
        let source_beams = right_beams.chain(left_beams).chain(up_beams).chain(down_beams);
        let max = source_beams.map(|source_beam| self.energize(source_beam).len()).max();
        max.unwrap()
    }
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
..//.|...."; // Add your test input here
        let expected_output = "51"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}