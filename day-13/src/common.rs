#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoilType {
    Rock,
    Ash,
}

pub struct Pattern {
    pub rows: Vec<Vec<SoilType>>
}

impl SoilType {
    pub fn parse(val: char) -> Self {
        match val {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Invalid value {val}")
        }
    }
}

impl Pattern {
    pub fn parse(pattern: &str) -> Self {
        let values = pattern.lines().map(|line| line.chars().map(SoilType::parse).collect()).collect();
        Self{ rows: values }
    }
}

pub type Line = Vec<SoilType>;

impl Pattern {

    pub fn columns(&self) -> Vec<Line> {
        (0..self.rows[0].len())
        .into_iter()
        .map(|column_index|{
            self.rows
            .iter()
            .map(|row| row[column_index])
            .collect::<Vec<SoilType>>()
        })
        .collect()
    }

    pub fn value<F>(&self, find_symmetry: F) -> usize
        where F: Fn (&[Line]) -> Option<usize> 
    {
        let row_symmetry = find_symmetry(&self.rows);
        return if let Some(value) = row_symmetry {
           value * 100
        } else {
            let column_symmetry = find_symmetry(&self.columns());
            column_symmetry.unwrap()
        };
    }
}