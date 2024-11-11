use std::{cmp::{max, min}, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Coordinates {
    x: u32,
    y: u32,
    z: u32
}

impl Coordinates {
    fn parse(value:&str) -> Self {
        let splitted = value.split(",").collect::<Vec<_>>();
        assert_eq!(3, splitted.len());
        return Self { 
            x: splitted[0].parse().unwrap(), 
            y: splitted[1].parse().unwrap(), 
            z: splitted[2].parse().unwrap() 
        }
    }

    fn go_down(&self, diff: u32) -> Self {
        Self { z: self.z - diff, ..*self}
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Brick {
    pub id: u32,
    start: Coordinates,
    end: Coordinates,
    min_x: u32, max_x: u32,
    min_y: u32, max_y: u32,
    min_z: u32, max_z: u32,
}

impl Brick {
    pub fn parse(id: u32, value: &str,) -> Self {
        let splitted = value.split("~").collect::<Vec<_>>();
        assert_eq!(2, splitted.len());
        let start = Coordinates::parse(splitted[0]);
        let end = Coordinates::parse(splitted[1]);

        Self::new(id, start, end)
    }

    fn new(id: u32, start: Coordinates, end: Coordinates) -> Self {
        let min_x = min(start.x, end.x);
        let max_x = max( start.x, end.x);

        let min_y = min(start.y, end.y);
        let max_y = max( start.y, end.y);

        let min_z = min(start.z, end.z);
        let max_z = max( start.z, end.z);

        Self { id, start, end, min_x, max_x, min_y, max_y, min_z, max_z }
    }

    fn intersect_in_2d(&self, other: &Self) -> bool {
        let x_intersect = !(other.max_x < self.min_x || other.min_x > self.max_x);
        let y_intersect = !(other.max_y < self.min_y || other.min_y > self.max_y);

        return x_intersect && y_intersect;
    }

    fn go_down(&self, floor: u32) -> Self {
        let z_diff = self.min_z - floor - 1;
        let new_start = self.start.go_down(z_diff);
        let new_end = self.end.go_down(z_diff);
        return Self::new(self.id, new_start, new_end);
    }
}

pub fn settle_bricks(mut bricks: Vec<Brick>) -> (Vec<Brick>, HashMap<u32, Vec<u32>>) {
    bricks.sort_by_key(|brick| brick.min_z);
    let mut settled_bricks = Vec::with_capacity(bricks.len());
    let mut supported_by: HashMap<u32, Vec<u32>> = HashMap::new() ;
    for brick in bricks {
        let mut max_z = None;
        // println!("{:?}", settled_bricks.iter().map(|b: &Brick| b.id).collect::<Vec<_>>());
        for settled in &settled_bricks {
            // println!("{:?}", settled);
            // println!("{}-{}:{}", brick.id, settled.id, brick.intersect_in_2d(&settled));
            if brick.intersect_in_2d(&settled) {
                if max_z.is_none_or(|max_z| max_z < settled.max_z) {
                    supported_by.insert(brick.id, vec![settled.id]);
                    max_z = Some(settled.max_z)
                } else if max_z.is_some_and(|max_z| max_z == settled.max_z) {
                    supported_by.get_mut(&brick.id).unwrap().push(settled.id);
                }
            }
        }
        // println!("{:?}", max_z);
        settled_bricks.push(brick.go_down(max_z.unwrap_or(0)));
    }
    return (settled_bricks, supported_by);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_coordinates() {
        let value = "0,0,2";
        let expected_coordinates = Coordinates {x: 0, y: 0, z: 2};
        assert_eq!(expected_coordinates, Coordinates::parse(value));
    }

    #[test]
    fn test_parse_brick() {
        let value = "1,0,1~1,2,1";
        let expected_brick = Brick::new(
            0,
            Coordinates {x: 1, y: 0, z: 1},
            Coordinates {x: 1, y: 2, z: 1},
        );
        assert_eq!(expected_brick, Brick::parse(0, value));
    }

    #[test]
    fn test_intersect_in_2d() {
        let brick_a = Brick::parse(0, "0,1,1~2,1,1");
        let brick_b = Brick::parse(1, "1,1,8~1,1,9");
        assert!(brick_b.intersect_in_2d(&brick_a));
    }

}