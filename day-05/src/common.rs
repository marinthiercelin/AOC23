use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

pub struct Mapping {
    pub source: Category,
    pub destination: Category,
    pub ranges: Vec<MappingRange>,
}



pub struct MappingRange {
    pub destination_start: u64,
    pub source_start: u64,
    pub range_length: u64,
}

impl Mapping {
    /// Parses the input and return a Mapping
    pub fn parse(mapping_input: &str) -> Mapping {
        let mut lines = mapping_input.lines();
        let (source, destination) = parse_mapping_header(lines.next().unwrap());
        let ranges = lines.map(|line| MappingRange::parse(line)).collect();
        Mapping { source, destination, ranges }
    }
}

impl Category {
    fn parse(category: &str) -> Category {
        match category {
            "seed" => Category::Seed,
            "soil" => Category::Soil,
            "fertilizer" => Category::Fertilizer,
            "water" => Category::Water,
            "light" => Category::Light,
            "temperature" => Category::Temperature,
            "humidity" => Category::Humidity,
            "location" => Category::Location,
            _ => panic!("Unknown category: {}", category),
        }
    }
}

fn parse_mapping_header(line: &str) -> (Category, Category) {
    let categories_part = line.split_whitespace().nth(0).unwrap();
    let mut categories = categories_part.split("-to-");
    let source = categories.next().unwrap();
    let destination = categories.next().unwrap();
    (Category::parse(source), Category::parse(destination))
}

impl MappingRange {
    fn parse(line: &str) -> MappingRange {
        let mut numbers = line.split_whitespace().map(|x| x.parse().unwrap());
        let destination_start = numbers.next().unwrap();
        let source_start = numbers.next().unwrap();
        let range_length = numbers.next().unwrap();
        MappingRange { destination_start, source_start, range_length }
    }
}

pub trait MappingCollection {
    fn get_mappings(&self)-> &HashMap<Category, Mapping>;

    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let mut mapped_value = seed;
        let mut category = &Category::Seed;
        let mappings = self.get_mappings();
        loop {
            let mapping = mappings.get(&category).unwrap();
            category = &mapping.destination;
            mapped_value = mapping.map_value(mapped_value);
            if category == &Category::Location {
                break;
            }
        }
        mapped_value
    }
}

impl Mapping {
    fn map_value(&self, value: u64) -> u64 {
        let mut mapped_value = value;
        for range in &self.ranges {
            if mapped_value >= range.source_start && mapped_value < range.source_start + range.range_length {
                mapped_value = range.destination_start + (mapped_value - range.source_start);
                break;
            }
        }
        mapped_value
    }
}