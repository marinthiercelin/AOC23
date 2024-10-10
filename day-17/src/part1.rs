use crate::common::{HeatMap, MoveRules};


pub fn run(input: &str) -> String {
    let heat_map = HeatMap::parse(input);
    let rules = MoveRules { min_blocks: 1, max_bocks: 3 };
    let minimal_heat_loss = heat_map.find_minimal_heat_loss(&rules);
    minimal_heat_loss.to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let expected_output = "102";
        assert_eq!(run(input), expected_output);
    }
}
