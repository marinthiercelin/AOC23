use std::collections::{HashMap, HashSet};

use crate::common::{self, Brick};

pub fn run(input: &str) -> String {
    let bricks = input.lines().enumerate().map(|(id, line)|Brick::parse(id as u32, line)).collect();
    let falling = count_falling(bricks);
    return falling.to_string()
}

pub fn count_falling(bricks: Vec<Brick>) -> usize {
    let (settled, supported_by) = common::settle_bricks(bricks);
    let mut count = 0;
    let mut falling_saved: HashMap<u32, HashSet<u32>> = HashMap::new();
    for brick in settled.iter().rev() {
        let mut falling = HashSet::new();
        falling.insert(brick.id);
        loop {
            let new_falling = supported_by
            .iter()
            .filter(|(supported,_)| !falling.contains(supported))
            .filter(|(_, supports)| supports.iter().all(|support|falling.contains(&support)))
            .map(|(&supported, _)| supported)
            .collect::<Vec<_>>();
            if new_falling.is_empty() {
                break;
            }
            new_falling.iter().for_each(|id|{
                falling.insert(*id);
                if let Some(saved) = falling_saved.get(id) {
                    saved
                    .iter()
                    .for_each(|saved_id|{falling.insert(*saved_id);});
                }
            });
        }
        falling.remove(&brick.id); // don't count the base brick
        count += falling.len();
        falling_saved.insert(brick.id, falling);
    }
    
    count
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
        let expected_output = "7";
        assert_eq!(run(input), expected_output);
    }

}