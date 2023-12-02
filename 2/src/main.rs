use env_logger;
use log;
use std::env;
use std::fs;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct HandfulCount {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
enum ParseHandfulCountError {
    UnknownColour,
    AmountParseError(ParseIntError),
}

impl From<ParseIntError> for ParseHandfulCountError {
    fn from(value: ParseIntError) -> Self {
        ParseHandfulCountError::AmountParseError(value)
    }
}

impl FromStr for HandfulCount {
    type Err = ParseHandfulCountError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut handful_count = HandfulCount {
            red: 0,
            green: 0,
            blue: 0,
        };

        let cube_counts = s.split(", ");

        for count in cube_counts {
            let mut cube_count = count.split(' ');

            let amount = cube_count.next().unwrap().parse()?;
            let cube_type = cube_count.next().unwrap();

            match cube_type {
                "red" => handful_count.red = amount,
                "green" => handful_count.green = amount,
                "blue" => handful_count.blue = amount,
                _ => return Err(ParseHandfulCountError::UnknownColour),
            };
        }

        Ok(handful_count)
    }
}

#[derive(Debug)]
enum ParseGameError {
    ParseIdError(ParseIntError),
    ParseSubsetsError(ParseHandfulCountError),
}

impl From<ParseIntError> for ParseGameError {
    fn from(value: ParseIntError) -> Self {
        ParseGameError::ParseIdError(value)
    }
}

impl From<ParseHandfulCountError> for ParseGameError {
    fn from(value: ParseHandfulCountError) -> Self {
        ParseGameError::ParseSubsetsError(value)
    }
}

#[derive(Debug)]
struct Game {
    id: u16,
    subsets: Vec<HandfulCount>,
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");

        const ID_START_INDEX: usize = 5;
        let id = split.next().unwrap()[ID_START_INDEX..].parse()?;

        let handfuls = split.next().unwrap();
        let subsets: Vec<HandfulCount> = handfuls
            .split("; ")
            .map(HandfulCount::from_str)
            .collect::<Result<Vec<HandfulCount>, ParseHandfulCountError>>()?;

        Ok(Game { id, subsets })
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    log::debug!("{}", input);

    let record: Vec<Game> = input
        .split('\n')
        .filter(|line| line != &"")
        .map(Game::from_str)
        .collect::<Result<Vec<Game>, ParseGameError>>()
        .unwrap();

    log::debug!("{:?}", record);

    let possible_games: Vec<Game> = record.into_iter().filter(is_valid_game).collect();
    let id_sum: u16 = possible_games.iter().map(|game| game.id).sum();

    println!("{}", id_sum);
}

fn is_valid_game(game: &Game) -> bool {
    const BAG_CONTENTS: HandfulCount = HandfulCount {
        red: 12,
        green: 13,
        blue: 14,
    };

    game.subsets.iter().all(|subset| {
        (subset.red <= BAG_CONTENTS.red)
            && (subset.green <= BAG_CONTENTS.green)
            && (subset.blue <= BAG_CONTENTS.blue)
    })
}
