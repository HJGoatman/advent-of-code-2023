mod contraption;

use contraption::Contraption;
use contraption::MirrorType;
use contraption::Position;
use contraption::SplitterType;
use contraption::Tile;

use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn get_adjacent_position(position: Position, direction: Direction) -> Option<Position> {
    match direction {
        Direction::Down => Some(Position {
            x: position.x,
            y: position.y + 1,
        }),
        Direction::Left => {
            if position.x == 0 {
                return None;
            }

            Some(Position {
                x: position.x - 1,
                y: position.y,
            })
        }
        Direction::Up => {
            if position.y == 0 {
                return None;
            }

            Some(Position {
                x: position.x,
                y: position.y - 1,
            })
        }
        Direction::Right => Some(Position {
            x: position.x + 1,
            y: position.y,
        }),
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Beam {
    position: Position,
    direction: Direction,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    log::debug!("{}", input);

    let contraption: Contraption = input.parse().unwrap();
    log::debug!("{}", contraption);

    let start_position = Position { x: 0, y: 0 };
    let start_direction = Direction::Right;
    let start_beam = Beam {
        position: start_position,
        direction: start_direction,
    };

    let energised_tile_positions = simulate_beam_through_contraption(&contraption, start_beam);
    let total_energised_tile_positions = energised_tile_positions.len();
    println!("{}", total_energised_tile_positions);
}

fn simulate_beam_through_contraption(contraption: &Contraption, start: Beam) -> HashSet<Position> {
    let mut distinct_beam_directions: HashSet<Beam> = HashSet::new();

    let mut current_beams = vec![start];

    while let Some(beam) = current_beams.pop() {
        if let Some(tile) = contraption.get(beam.position) {
            log::trace!("{:?}", beam);

            distinct_beam_directions.insert(beam);

            let next_beams = get_next_beams(beam, tile);

            for next_beam in next_beams {
                if !distinct_beam_directions.contains(&next_beam) {
                    current_beams.push(next_beam);
                }
            }
        }
    }

    let energised_tile_positions = distinct_beam_directions
        .into_iter()
        .map(|beam| beam.position)
        .collect();
    energised_tile_positions
}

fn get_next_beams(beam: Beam, tile: Tile) -> Vec<Beam> {
    let next_directions = match (beam.direction, tile) {
        (Direction::Up | Direction::Down, Tile::Splitter(SplitterType::Horizontal)) => {
            vec![Direction::Left, Direction::Right]
        }
        (Direction::Left | Direction::Right, Tile::Splitter(SplitterType::Vertical)) => {
            vec![Direction::Up, Direction::Down]
        }
        (Direction::Up, Tile::Mirror(MirrorType::Forward)) => vec![Direction::Right],
        (Direction::Up, Tile::Mirror(MirrorType::Backward)) => vec![Direction::Left],
        (Direction::Down, Tile::Mirror(MirrorType::Forward)) => vec![Direction::Left],
        (Direction::Down, Tile::Mirror(MirrorType::Backward)) => vec![Direction::Right],
        (Direction::Left, Tile::Mirror(MirrorType::Forward)) => vec![Direction::Down],
        (Direction::Left, Tile::Mirror(MirrorType::Backward)) => vec![Direction::Up],
        (Direction::Right, Tile::Mirror(MirrorType::Forward)) => vec![Direction::Up],
        (Direction::Right, Tile::Mirror(MirrorType::Backward)) => vec![Direction::Down],
        (direction, _) => vec![direction],
    };

    next_directions
        .into_iter()
        .map(|direction| (get_adjacent_position(beam.position, direction), direction))
        .filter(|(maybe_position, _)| maybe_position.is_some())
        .map(|(position, direction)| Beam {
            position: position.unwrap(),
            direction,
        })
        .collect()
}
