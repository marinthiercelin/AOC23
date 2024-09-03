use std::collections::HashMap;

use crate::common::{Category, Mapping, MappingCollection};

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



impl Almanac {
    /// Parses the input and return an Almanac
    fn parse(input: &str) -> Almanac {
        let mut elements = input.split("\n\n");
        let seeds = elements.next().unwrap().split(": ").nth(1).unwrap().split(" ").map(|s| s.parse().unwrap()).collect();
        let mappings = elements.map(|mapping_input| Mapping::parse(mapping_input)).map(|m| (m.source.clone(), m)).collect();
        Almanac { seeds, mappings }
    }
}

impl MappingCollection for Almanac {
    fn get_mappings(&self) -> &HashMap<Category, Mapping> {
        &self.mappings
    }
}


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

#[cfg(test)]
mod bench  {
    
    use std::fs;

    use rand::Rng;

    use super::*;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_map_seed_to_location(b: &mut Bencher) {
        let almanac = Almanac::parse(fs::read_to_string("input.txt").unwrap().as_str());
        let mut rng = rand::thread_rng();
        let mut min_location: Option<u64> = None;
        b.iter(||{
            let seed = almanac.seeds[rng.gen_range(0..almanac.seeds.len())];
            let location = almanac.map_seed_to_location(seed);
            min_location = Some(min_location.map_or(location, |min| min.min(location)));
        });
        test::black_box(min_location.unwrap());
    }
}