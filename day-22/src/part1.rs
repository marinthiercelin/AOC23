use std::collections::HashSet;

use crate::common::{self, Brick};

pub fn run(input: &str) -> String {
    let bricks = input.lines().enumerate().map(|(id, line)|Brick::parse(id as u32, line)).collect();
    let disintigrated = count_disintegrated(bricks);
    return disintigrated.to_string()
}

fn count_disintegrated(bricks: Vec<Brick>) -> usize {
    let num_bricks = bricks.len();
    let (_, supported_by) = common::settle_bricks(bricks);
    // println!("{:?}", settled);
    // println!("{:?}", supported_by);
    let structural_bricks = supported_by.values().filter(|supports| supports.len() == 1).collect::<HashSet<_>>();
    return num_bricks - structural_bricks.len();
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        let expected_output = "5";
        assert_eq!(run(input), expected_output);
    }

}
