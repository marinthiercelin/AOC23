use std::collections::HashMap;

pub fn run(input: &str) -> String {
    let almanac = Almanac::parse(input);
    let locations = almanac.seeds.iter().map(|seed| almanac.map_seed_to_location(*seed)).collect::<Vec<u64>>();
    let closest_location = locations.into_iter().min().unwrap();
    closest_location.to_string()
}

struct Almanac {
    seeds: Vec<u64>,
    mappings: HashMap<Category, Mapping>
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

struct Mapping {
    source: Category,
    destination: Category,
    ranges: Vec<MappingRange>,
}



struct MappingRange {
    destination_start: u64,
    source_start: u64,
    range_length: u64,
}

impl Almanac {
    /// Parses the input and return an Almanac
    fn parse(input: &str) -> Almanac {
        let mut elements = input.split("\n\n");
        let seeds = elements.next().unwrap().split(": ").nth(1).unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
        let mappings = elements.map(|mapping_input| Mapping::parse(mapping_input)).map(|m| (m.source.clone(), m)).collect();
        Almanac { seeds, mappings }
    }

    fn map_seed_to_location(&self, seed: u64) -> u64 {
        let mut mapped_value = seed;
        let mut category = &Category::Seed;
        loop {
            let mapping = self.mappings.get(&category).unwrap();
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
    /// Parses the input and return a Mapping
    fn parse(mapping_input: &str) -> Mapping {
        let mut lines = mapping_input.lines();
        let (source, destination) = parse_mapping_header(lines.next().unwrap());
        let ranges = lines.map(|line| MappingRange::parse(line)).collect();
        Mapping { source, destination, ranges }
    }
    
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"; // Add your test input here
        let expected_output = "35"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}
