use env_logger;
use log;
use std::env;
use std::fs;
use std::num::ParseIntError;

struct Race {
    time: u64,
    record_distance: u64,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn parse_document_row(s: &str) -> Result<Vec<u64>, ParseIntError> {
    s.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>())
        .collect()
}

fn parse_part_2_document_row(s: &str) -> Result<u64, ParseIntError> {
    s.split_whitespace().skip(1).collect::<String>().parse()
}

#[derive(Debug)]
struct ImpossibleToWinError;

fn main() {
    env_logger::init();

    let input = load_input();
    let lines: Vec<String> = input.split('\n').map(|line| line.to_string()).collect();
    let times = parse_document_row(lines.get(0).unwrap()).unwrap();
    let record_distances = parse_document_row(lines.get(1).unwrap()).unwrap();

    let races: Vec<Race> = times
        .into_iter()
        .zip(record_distances.into_iter())
        .map(|(time, record_distance)| Race {
            time,
            record_distance,
        })
        .collect();

    let ways_of_winning_each_race: Vec<u64> = races
        .iter()
        .map(|race| {
            calculate_number_of_ways_of_winning(race.record_distance + 1, race.time)
                .map(|v| v.into())
                .ok_or(ImpossibleToWinError)
        })
        .collect::<Result<Vec<u64>, ImpossibleToWinError>>()
        .unwrap();

    log::debug!("{:?}", ways_of_winning_each_race);
    let margin_of_error: u64 = ways_of_winning_each_race.iter().product();

    println!("{}", margin_of_error);

    let time: u64 = parse_part_2_document_row(lines.get(0).unwrap()).unwrap();
    let record_distance: u64 = parse_part_2_document_row(lines.get(1).unwrap()).unwrap();

    let race = Race {
        time,
        record_distance,
    };
    let ways_of_winning =
        calculate_number_of_ways_of_winning(race.record_distance, race.time).unwrap();
    println!("{}", ways_of_winning);
}

fn calculate_number_of_ways_of_winning(target_distance: u64, total_time: u64) -> Option<u64> {
    const STARTING_SPEED_MILLIMETERS_PER_MILLISECOND: f64 = 0.;
    const CHARGE_BUTTON_SPEED_INCREASE_MILLIMETERS_PER_SECOND: f64 = 1.;

    let total_time: f64 = total_time as f64;
    let target_distance: f64 = target_distance as f64;

    let a: f64 = -1.;
    let b: f64 = total_time * CHARGE_BUTTON_SPEED_INCREASE_MILLIMETERS_PER_SECOND;
    let c: f64 = STARTING_SPEED_MILLIMETERS_PER_MILLISECOND - target_distance;

    log::trace!("(a, b, c): ({}, {}, {})", a, b, c);

    let discriminant: f64 = b.powf(2.) - 4. * a * c;

    log::trace!("Discriminant: {}", discriminant);

    if discriminant.is_sign_negative() {
        return None;
    }

    let root_discriminant = discriminant.sqrt();
    log::trace!("Root Discriminant: {}", root_discriminant);
    let denominante = 2. * a;

    let min_time_taken_holding_button = (-b + root_discriminant) / denominante;
    let max_time_taken_holding_button = (-b - root_discriminant) / denominante;

    log::trace!(
        "{}, {}",
        min_time_taken_holding_button,
        max_time_taken_holding_button
    );

    let integer_max_time: u64 = max_time_taken_holding_button.ceil() as u64;
    let integer_min_time: u64 = min_time_taken_holding_button.ceil() as u64;

    return Some(integer_max_time - integer_min_time);
}
