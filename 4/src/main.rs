use env_logger;
use log;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Numbers {
    values: Vec<u32>,
}

impl FromStr for Numbers {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parse_result: Result<Vec<u32>, ParseIntError> =
            s.split_whitespace().map(|num| num.parse()).collect();

        let values = parse_result?;

        Ok(Numbers { values })
    }
}

#[derive(Debug)]
struct Scratchcard {
    id: u8,
    winning_numbers: Numbers,
    player_numbers: Numbers,
}

#[derive(Debug)]
enum ParseScratchcardError {
    ParseScratchcardIdError(ParseIntError),
    ParseWinningNumbersError(ParseIntError),
    ParsePlayerNumbersError(ParseIntError),
}

impl FromStr for Scratchcard {
    type Err = ParseScratchcardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");

        let id = split
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .map_err(|e| ParseScratchcardError::ParseScratchcardIdError(e))?;

        let number_side = split.next().unwrap();
        let mut number_split = number_side.split(" | ");

        let winning_numbers = number_split
            .next()
            .unwrap()
            .parse()
            .map_err(|parse_int_error| {
                ParseScratchcardError::ParseWinningNumbersError(parse_int_error)
            })?;

        let player_numbers = number_split
            .next()
            .unwrap()
            .parse()
            .map_err(|parse_int_error| {
                ParseScratchcardError::ParsePlayerNumbersError(parse_int_error)
            })?;

        Ok(Scratchcard {
            id,
            winning_numbers,
            player_numbers,
        })
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let lines: Vec<String> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(|line| line.to_string())
        .collect();

    let scratchcards: Vec<Scratchcard> = lines
        .iter()
        .map(|s| Scratchcard::from_str(s))
        .collect::<Result<Vec<Scratchcard>, ParseScratchcardError>>()
        .unwrap();

    log::debug!("{:?}", scratchcards);

    let total_points: u32 = scratchcards.iter().map(get_scratchcard_points).sum();
    println!("{}", total_points);

    let matches: Vec<usize> = scratchcards.iter().map(get_number_of_matches).collect();
    log::debug!("Matches: {:?}", matches);

    let copies: Vec<u32> = matches.iter().enumerate().rev().fold(
        vec![1; scratchcards.len()],
        |mut cards_won, (index, number_of_matches)| {
            let max_index = scratchcards.len() - 1;

            if index == max_index {
                return cards_won;
            }

            let start_index = index + 1;
            let last_index = (start_index + number_of_matches).min(max_index + 1);

            let new_cards: u32 = cards_won
                .get((index + 1)..last_index)
                .unwrap()
                .into_iter()
                .cloned()
                .sum();

            cards_won[index] += new_cards;
            cards_won
        },
    );

    log::debug!("Copies: {:?}", copies);

    let total_scratchcards: u32 = copies.iter().sum();
    println!("{}", total_scratchcards);
}

fn get_scratchcard_points(scratchcard: &Scratchcard) -> u32 {
    let number_of_matches = get_number_of_matches(scratchcard);

    if number_of_matches == 0 {
        return 0;
    }

    const BASE_2: u32 = 2;
    let points = BASE_2.pow((number_of_matches as u32) - 1);
    points
}

fn get_number_of_matches(scratchcard: &Scratchcard) -> usize {
    scratchcard
        .player_numbers
        .values
        .iter()
        .filter(|player_number| scratchcard.winning_numbers.values.contains(player_number))
        .count()
}
