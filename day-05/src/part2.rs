use std::collections::HashMap;

use crate::common::{Category, Mapping, MappingCollection, MappingRange};

pub fn run(input: &str) -> String {
    let almanac= Almanac::parse(input);
    let total_seeds = almanac.seeds_ranges.iter().map(|range| range.length).sum::<u64>();
    dbg!(total_seeds);
    let mut minimum_location: Option<u64> = None;
    let mut count = 0;
    for seed_range in almanac.seeds_ranges.iter() {
        for seed in seed_range.start..seed_range.start + seed_range.length {
            let location = almanac.map_seed_to_location(seed);
            minimum_location = Some(minimum_location.map_or(location, |minimum| minimum.min(location)));
        }
        count += seed_range.length;
        let progress  = (count as f64 * 100.0)/ total_seeds as f64;
        println!("{count}/{total_seeds} ({progress:.2}%) seeds processed");
    }
    minimum_location.unwrap().to_string()
}

struct Almanac {
    seeds_ranges: Vec<Range>,
    mappings: HashMap<Category, Mapping>
}

struct Range {
    start: u64,
    length: u64,
}

impl Almanac {
    /// Parses the input and return an Almanac
    fn parse(input: &str) -> Almanac {
        let mut elements = input.split("\n\n");
        let seed_numbers: Vec<u64> = elements.next().unwrap().split(": ").nth(1).unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
        let mut seeds_ranges = Vec::new();
        assert!(seed_numbers.len() % 2 == 0);
        for seed_pair_index in (0..seed_numbers.len()).step_by(2) {
            let seed_range = Range { start: seed_numbers[seed_pair_index], length: seed_numbers[seed_pair_index + 1] };
            seeds_ranges.push(seed_range);
        }
        let mappings = elements.map(|mapping_input| Mapping::parse(mapping_input)).map(|m| (m.source.clone(), m)).collect();
        Almanac { seeds_ranges, mappings }
    }
}

impl MappingCollection for Almanac {
    fn get_mappings(&self) -> &HashMap<Category, Mapping> {
        &self.mappings
    }
}

// impl Mapping {
//     fn map_range(&self, range: &Range)-> Vec<Range> {
        
//     }
// }


// impl MappingRange {
//     fn map_range(&self, range: &Range) -> Option<Range> {
        
//     }
// }

// impl Range {

// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = ""; // Add your test input here
        let expected_output = ""; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}