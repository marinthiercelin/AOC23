use regex::Regex;

pub fn parse_and_analyze_engine<'a>(input: &'a str) -> (Vec<Number>, Vec<Symbol<'a>>) {
    let mut part_numbers = Vec::new();
    let mut gears = Vec::new();
    let mut previous_row_numbers = Vec::new();
    let mut previous_row_symbols = Vec::new();
    for line in input.lines() {
        let (mut numbers, mut symbols) = parse_row(line);
        for symbol in &mut symbols {
            for number in &mut previous_row_numbers {
                if are_adjacent(number, symbol) {
                    number.is_part_number = true;
                    symbol.neighbors.push(number.value);
                }
            }
            for number in &mut numbers {
                if are_adjacent(number, symbol) {
                    number.is_part_number = true;
                    symbol.neighbors.push(number.value);
                }
            }
        }
        for symbol in &mut previous_row_symbols {
            for number in &mut numbers {
                if are_adjacent(number, symbol) {
                    number.is_part_number = true;
                    symbol.neighbors.push(number.value);
                }
            }
        }
        let mut new_part_numbers: Vec<Number> = previous_row_numbers.into_iter().filter(|n| n.is_part_number).collect();
        part_numbers.append(&mut new_part_numbers);
        let mut new_gears = previous_row_symbols.into_iter().filter(|s| s.character == "*" && s.neighbors.len() == 2).collect();
        gears.append(&mut new_gears);
        previous_row_numbers = numbers;
        previous_row_symbols = symbols;
    }
    let mut new_part_numbers: Vec<Number> = previous_row_numbers.into_iter().filter(|n| n.is_part_number).collect();
    part_numbers.append(&mut new_part_numbers);
    let mut new_gears = previous_row_symbols.into_iter().filter(|s| s.character == "*" && s.neighbors.len() == 2).collect();
    gears.append(&mut new_gears);
    (part_numbers, gears)
}

fn are_adjacent(number: &Number, symbol: &Symbol) -> bool {
    let (number_start, number_end) = number.coordinates;
    let number_start = number_start as i32;
    let number_end = number_end as i32;
    let symbol_index = symbol.index as i32;
    symbol_index >= number_start - 1 && symbol_index <= number_end
}

pub struct Number {
    pub value: u32,
    pub is_part_number: bool,
    pub coordinates: (usize, usize),
}

pub struct Symbol<'a> {
    pub index: usize,
    pub character: &'a str,
    pub neighbors: Vec<u32>,
}

fn parse_row<'a>(line: &'a str) -> (Vec<Number>, Vec<Symbol<'a>>) {
    let re = Regex::new(r"\d+").unwrap();
    let numbers = re.find_iter(line).map(|m| {
        Number {
            value: m.as_str().parse().unwrap(),
            is_part_number: false,
            coordinates: (m.start(), m.end()),
        }
    }).collect();
    let re = Regex::new(r"[^.\d]").unwrap();
    let symbols = re.find_iter(line).map(|m| {
        Symbol {
            index: m.start(),
            character: m.as_str(),
            neighbors: Vec::new(),
        }
    }).collect();
    (numbers, symbols)
} 