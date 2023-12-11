use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Field {
    tiles: Vec<Tile>,
    width: usize,
}

impl Field {
    fn get(&self, position: Position) -> Option<Tile> {
        if position.x > self.width || position.y > self.width {
            return None;
        }

        let lookup_index = position.y * self.width + position.x;

        Some(self.tiles[lookup_index])
    }

    fn get_start_position(&self) -> Position {
        let (index, _) = self
            .tiles
            .iter()
            .enumerate()
            .find(|(_i, tile)| **tile == Tile::StartingPosition)
            .unwrap();

        let x = index % self.width;
        let y = index / self.width;

        Position { x, y }
    }
}

#[derive(Debug)]
enum FieldError {
    ParseTileError(ParseTileError),
    NotSquareField,
}

impl FromStr for Field {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = s.split('\n').map(|line| line.to_string()).collect();
        let rows: Vec<Vec<Tile>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<Tile>, ParseTileError>>()
            })
            .collect::<Result<Vec<Vec<Tile>>, ParseTileError>>()
            .map_err(FieldError::ParseTileError)?;

        let width = rows.first().unwrap().len();
        if rows.iter().any(|row| row.len() != width) {
            return Err(FieldError::NotSquareField);
        }

        let tiles = rows.into_iter().flatten().collect();

        Ok(Field { tiles, width })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    NorthEastBend,
    NorthWestBend,
    SouthWestBend,
    SouthEastBend,
    Ground,
    StartingPosition,
}

#[derive(Debug)]
enum ParseTileError {
    UnknownTile,
}

impl TryFrom<char> for Tile {
    type Error = ParseTileError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::VerticalPipe),
            '-' => Ok(Tile::HorizontalPipe),
            'L' => Ok(Tile::NorthEastBend),
            'J' => Ok(Tile::NorthWestBend),
            '7' => Ok(Tile::SouthWestBend),
            'F' => Ok(Tile::SouthEastBend),
            '.' => Ok(Tile::Ground),
            'S' => Ok(Tile::StartingPosition),
            _ => Err(ParseTileError::UnknownTile),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
struct State {
    position: Position,
    direction: Direction,
    distance: u32,
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let field: Field = input.parse().unwrap();

    log::debug!("Field: {:?}", field);

    let farthest_distance = find_loop(&field);

    println!("{}", farthest_distance);
}

fn find_loop(field: &Field) -> u32 {
    let start = field.get_start_position();

    let (start_1, start_2) = find_connected_pipes(field, &start).unwrap();
    log::debug!("{:?}, {:?}", start_1, start_2);

    let mut current_1 = start_1;
    let mut current_2 = start_2;

    while current_1.position != current_2.position {
        current_1 = get_next_state(field, current_1);
        current_2 = get_next_state(field, current_2);
    }

    current_1.distance
}

fn get_next_state(field: &Field, state: State) -> State {
    let current_tile = field.get(state.position).unwrap();
    let next_direction = traverse_pipe(current_tile, state.direction).unwrap();
    let next_position = travel_direction(state.position, next_direction).unwrap();

    State {
        position: next_position,
        direction: next_direction,
        distance: state.distance + 1,
    }
}

fn find_connected_pipes(field: &Field, position: &Position) -> Option<(State, State)> {
    let mut direction_combinations = Vec::new();

    const DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    for (i, first_direction) in DIRECTIONS.iter().enumerate().take(DIRECTIONS.len() - 1) {
        for second_direction in DIRECTIONS.iter().skip(i + 1) {
            direction_combinations.push((*first_direction, *second_direction))
        }
    }

    log::trace!("{:?}", direction_combinations);

    for (direction_1, direction_2) in direction_combinations {
        if let (Some(new_position_1), Some(new_position_2)) = (
            travel_direction(*position, direction_1),
            travel_direction(*position, direction_2),
        ) {
            if let (Some(new_tile_1), Some(new_tile_2)) =
                (field.get(new_position_1), field.get(new_position_2))
            {
                if traverse_pipe(new_tile_1, direction_1).is_some()
                    && traverse_pipe(new_tile_2, direction_2).is_some()
                {
                    return Some((
                        State {
                            position: new_position_1,
                            direction: direction_1,
                            distance: 1,
                        },
                        State {
                            position: new_position_2,
                            direction: direction_2,
                            distance: 1,
                        },
                    ));
                }
            }
        }
    }

    None
}

fn traverse_pipe(tile: Tile, direction: Direction) -> Option<Direction> {
    match (tile, direction) {
        (Tile::Ground | Tile::StartingPosition, _) => None,
        (Tile::VerticalPipe, Direction::North) => Some(Direction::North),
        (Tile::VerticalPipe, Direction::South) => Some(Direction::South),
        (Tile::HorizontalPipe, Direction::West) => Some(Direction::West),
        (Tile::HorizontalPipe, Direction::East) => Some(Direction::East),
        (Tile::NorthEastBend, Direction::South) => Some(Direction::East),
        (Tile::NorthEastBend, Direction::West) => Some(Direction::North),
        (Tile::NorthWestBend, Direction::South) => Some(Direction::West),
        (Tile::NorthWestBend, Direction::East) => Some(Direction::North),
        (Tile::SouthWestBend, Direction::North) => Some(Direction::West),
        (Tile::SouthWestBend, Direction::East) => Some(Direction::South),
        (Tile::SouthEastBend, Direction::North) => Some(Direction::East),
        (Tile::SouthEastBend, Direction::West) => Some(Direction::South),
        (_, _) => None,
    }
}

fn travel_direction(position: Position, direction: Direction) -> Option<Position> {
    match (direction, position.y > 0, position.x > 0) {
        (Direction::North, false, _) => None,
        (Direction::North, true, _) => Some(Position {
            x: position.x,
            y: position.y - 1,
        }),
        (Direction::South, _, _) => Some(Position {
            x: position.x,
            y: position.y + 1,
        }),
        (Direction::West, _, false) => None,
        (Direction::West, _, true) => Some(Position {
            x: position.x - 1,
            y: position.y,
        }),
        (Direction::East, _, _) => Some(Position {
            x: position.x + 1,
            y: position.y,
        }),
    }
}
