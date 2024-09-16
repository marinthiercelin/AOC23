pub struct History {
    pub values: Vec<i32>,
}

impl History {
    pub fn parse(line: &str) -> Self {
        let values = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        Self { values }
    }
}