use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::panic;
use std::str::FromStr;
use std::usize;

use env_logger;
use log;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum SchematicPart {
    Number(u32),
    Symbol(SymbolType),
}

#[derive(Debug)]
enum SymbolType {
    Star,
    Other,
}

#[derive(Debug)]
struct EngineSchematic {
    parts: Vec<SchematicPart>,
    part_lookup: HashMap<Position, usize>,
}

#[derive(Debug)]
enum ParseEngineSchematicError {}

impl FromStr for EngineSchematic {
    type Err = ParseEngineSchematicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut part_lookup = HashMap::new();
        let mut parts = Vec::new();

        let mut x = 0;
        let mut y = 0;

        let mut input_iter = s.chars().peekable();
        while let Some(character) = input_iter.next() {
            if character == '\n' {
                y += 1;
                x = 0;
                continue;
            }

            if character == '.' {
                x += 1;
                continue;
            }

            let new_part_index = parts.len();

            if character.is_numeric() {
                let mut digit_queue = vec![character];

                while let Some(digit) = input_iter.next_if(|maybe_digit| maybe_digit.is_numeric()) {
                    digit_queue.push(digit);
                }

                let part_number = digit_queue
                    .iter()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                parts.push(SchematicPart::Number(part_number));

                let number_length = digit_queue.len();

                for i in 0..number_length {
                    let position = Position { x: x + i, y };

                    part_lookup.insert(position, new_part_index);
                }

                x += number_length;
                continue;
            }

            // Assume the character to be a symbol.
            let symbol_type = if character == '*' {
                SymbolType::Star
            } else {
                SymbolType::Other
            };
            parts.push(SchematicPart::Symbol(symbol_type));
            part_lookup.insert(Position { x, y }, new_part_index);
            x += 1;
        }

        Ok(EngineSchematic { parts, part_lookup })
    }
}

#[derive(Debug)]
struct Gear {
    gear_ratio: u32,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    log::debug!("Input:\n{}", &input);

    let engine_schematic: EngineSchematic = input.parse().unwrap();
    log::debug!("Schematic: {:?}", &engine_schematic);

    let part_numbers = get_part_numbers(&engine_schematic);
    log::debug!("Part Numbers: {:?}", part_numbers);

    let part_numbers_sum: u32 = part_numbers.iter().sum();
    println!("{}", part_numbers_sum);

    let gears = get_gears(&engine_schematic);
    log::debug!("Gears: {:?}", gears);

    let gear_ratio_sum: u32 = gears.iter().map(|gear| gear.gear_ratio).sum();
    println!("{}", gear_ratio_sum);
}

fn get_part_numbers(schematic: &EngineSchematic) -> Vec<u32> {
    let mut part_indexes: HashSet<usize> = HashSet::new();

    for (position, part_index) in schematic.part_lookup.iter() {
        if let SchematicPart::Symbol(_) = schematic.parts[*part_index] {
            let adjacent_part_indexes = get_adjacent_part_indexes(&schematic.part_lookup, position);
            part_indexes.extend(adjacent_part_indexes)
        }
    }

    part_indexes
        .into_iter()
        .map(|index| {
            if let SchematicPart::Number(part_number) = schematic.parts[index] {
                part_number
            } else {
                // I've got lucky with my input as no symbols are adjacent.
                panic!("Should be a part number!");
            }
        })
        .collect()
}

fn get_adjacent_part_indexes(
    part_lookup: &HashMap<Position, usize>,
    position: &Position,
) -> Vec<usize> {
    let mut adjacent_part_indexes = HashSet::new();

    for dx in -1..2 {
        let x = position.x as i32 + dx;

        if x.is_negative() {
            continue;
        }

        for dy in -1..2 {
            let y = position.y as i32 + dy;

            if y.is_negative() {
                continue;
            }

            if (dx == 0) && (dy == 0) {
                continue;
            }

            let search_position = Position {
                x: x as usize,
                y: y as usize,
            };

            match part_lookup.get(&search_position) {
                Some(part_index) => adjacent_part_indexes.insert(*part_index),
                _ => continue,
            };
        }
    }

    adjacent_part_indexes.into_iter().collect()
}

fn get_gears(schematic: &EngineSchematic) -> Vec<Gear> {
    let mut gears = Vec::new();

    for (position, part_index) in schematic.part_lookup.iter() {
        if let SchematicPart::Symbol(SymbolType::Star) = schematic.parts[*part_index] {
            let adjacent_part_indexes = get_adjacent_part_indexes(&schematic.part_lookup, position);

            if adjacent_part_indexes.len() != 2 {
                continue;
            }

            let gear_ratio: u32 = adjacent_part_indexes
                .into_iter()
                .map(|index| {
                    if let SchematicPart::Number(part_number) = schematic.parts[index] {
                        part_number
                    } else {
                        panic!("Should be a part number!");
                    }
                })
                .product();

            gears.push(Gear { gear_ratio });
        }
    }

    gears
}
