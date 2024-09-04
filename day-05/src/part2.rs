use std::{collections::HashMap, ops::Range};

use crate::common::{Category, Mapping, MappingCollection, MappingRange};

pub fn run(input: &str) -> String {
    let almanac= Almanac::parse(input);
    let mut minimal_location: Option<u64> = None;
    for seed_range in almanac.seeds_ranges.iter() {
        let location_ranges = almanac.map_seed_range_to_location_ranges(seed_range);
        let loop_minimal_location = location_ranges.into_iter().map(|range| range.start).min().unwrap();
        minimal_location = match minimal_location {
            None => Some(loop_minimal_location),
            Some(minimal_location) => Some(minimal_location.min(loop_minimal_location))
        };
    }
    minimal_location.unwrap().to_string()
}

struct Almanac {
    seeds_ranges: Vec<Range<u64>>,
    mappings: HashMap<Category, Mapping>
}

impl Almanac {
    /// Parses the input and return an Almanac
    fn parse(input: &str) -> Almanac {
        let mut elements = input.split("\n\n");
        let seed_numbers: Vec<u64> = elements.next().unwrap().split(": ").nth(1).unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
        let mut seeds_ranges = Vec::new();
        assert!(seed_numbers.len() % 2 == 0);
        for seed_pair_index in (0..seed_numbers.len()).step_by(2) {
            let seed_range = Range { start: seed_numbers[seed_pair_index], end: seed_numbers[seed_pair_index] + seed_numbers[seed_pair_index + 1] };
            seeds_ranges.push(seed_range);
        }
        let mappings = elements.map(|mapping_input| Mapping::parse(mapping_input)).map(|m| (m.source.clone(), m)).collect();
        Almanac { seeds_ranges, mappings }
    }

    fn map_seed_range_to_location_ranges(&self, seed_range: &Range<u64>) -> Vec<Range<u64>> {
        let mut mapped_ranges = vec![seed_range.clone()];
        let mut category = &Category::Seed;
        let mappings = self.get_mappings();
        loop {
            let mapping = mappings.get(&category).unwrap();
            category = &mapping.destination;
            let new_mapped_ranges = mapped_ranges.iter().flat_map(|range| mapping.map_range(range)).collect();
            mapped_ranges = new_mapped_ranges;
            mapped_ranges.sort_by_key(|range|{range.start});
            if category == &Category::Location {
                break;
            }
        }
        mapped_ranges
    }
}

impl MappingCollection for Almanac {
    fn get_mappings(&self) -> &HashMap<Category, Mapping> {
        &self.mappings
    }
}

impl Mapping {
    fn map_range(&self, range: &Range<u64>)-> Vec<Range<u64>> {
        let mut not_mapped = vec![range.clone()];
        let mut mapped = Vec::new();
        for mapping_range in self.ranges.iter() {
            let mut next_not_mapped = Vec::new();
            for value_range in not_mapped {
                if let Some((new_not_mapped, new_mapped)) = mapping_range.map_range(&value_range) {
                    next_not_mapped.extend(new_not_mapped);
                    mapped.push(new_mapped);
                } else {
                    next_not_mapped.push(value_range);
                }
            }
            not_mapped = next_not_mapped;
        }
        mapped.extend(not_mapped);
        mapped
    }
}

impl MappingRange {


    fn map_range(&self, value_range: &Range<u64>) -> Option<(Vec<Range<u64>>, Range<u64>)> {
        let mapping_range = self.source_range();
        let intersect_start = mapping_range.start.max(value_range.start);
        let intersect_end = mapping_range.end.min(value_range.end);
        if intersect_start < intersect_end {
            let mut not_mapped = Vec::new();
            if value_range.start < intersect_start {
                not_mapped.push(Range { start: value_range.start, end: intersect_start });
            }
            if value_range.end > intersect_end {
                not_mapped.push(Range { start: intersect_end, end: value_range.end });
            }
            let mapping_offset: i64 = self.offset();
            let mapped = Range { start: ( intersect_start as i64 + mapping_offset ) as u64, end: (intersect_end as i64 + mapping_offset) as u64 };
            Some((not_mapped, mapped))
        } else {
            None
        }
    }
}

impl MappingRange {
    pub fn source_range(&self) -> Range<u64> {
        self.source_start..self.source_start + self.range_length
    }

    fn offset(&self) -> i64 {
        self.destination_start as i64 - self.source_start as i64
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
        let expected_output = "46"; // Add the expected output here
        assert_eq!(run(input), expected_output);
    }

    // Add more tests here
}