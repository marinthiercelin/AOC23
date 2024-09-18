#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UniversePoint {
    Empty,
    Galaxy
}

#[derive(Debug)]
pub struct Universe {
    pub grid: Vec<Vec<UniversePoint>>,
}

impl Universe {
    pub fn parse(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.chars().map(|c| {
                    match c {
                        '.' => UniversePoint::Empty,
                        '#' => UniversePoint::Galaxy,
                        _ => panic!("Invalid character in input")
                    }
                }).collect()
            })
            .collect();
        Self { grid }
    }

    pub fn get_empty_columns(&self) -> Vec<usize> {
        (0..self.grid[0].len()).filter(|column| self.grid.iter().all(|row| row[*column] == UniversePoint::Empty)).collect::<Vec<_>>()
    }

    pub fn get_empty_rows(&self) -> Vec<usize> {
        self.grid.iter().enumerate().filter(|(_, row)| row.iter().all(|&point| point == UniversePoint::Empty)).map(|(index, _)| index).collect::<Vec<_>>()
    }

    pub fn get_galaxy_positions(&self) -> Vec<GalaxyPosition> {
        self.grid.iter().enumerate().map(|(row_index, row)| {
            row.iter().enumerate().filter(|(_, &point)| point == UniversePoint::Galaxy).map(|(column_index, _)| (row_index, column_index)).collect::<Vec<_>>()
        }).flatten().collect()
    }
}

pub fn get_shortest_distance(galaxy: &GalaxyPosition, other_galaxy: &GalaxyPosition) -> usize {
    let &(x1, y1) = galaxy;
    let &(x2, y2) = other_galaxy;
    ((x1 as isize - x2 as isize).abs() + (y1 as isize - y2 as isize).abs())as usize
}

pub type GalaxyPosition = (usize, usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_function() {
        let galaxy1 = (6, 1);
        let galaxy2 = (11, 5);
        let distance = get_shortest_distance(&galaxy1, &galaxy2);
        assert_eq!(9, distance);
    }
}