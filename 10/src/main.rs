use std::collections::HashSet;
use std::env;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Field {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Field {
    fn get_index(&self, position: Position) -> usize {
        position.y * self.width + position.x
    }

    fn get(&self, position: Position) -> Option<Tile> {
        if position.x >= self.width || position.y >= self.height {
            return None;
        }

        let lookup_index = self.get_index(position);

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

    fn filter(&mut self, positions: &[Position]) {
        let indexes_to_keep: Vec<usize> = positions
            .into_iter()
            .map(|position| self.get_index(*position))
            .collect();

        self.tiles.iter_mut().enumerate().for_each(|(i, v)| {
            if !indexes_to_keep.contains(&i) {
                *v = Tile::Ground
            }
        });
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
        let lines: Vec<String> = s
            .split('\n')
            .filter(|line| line != &"")
            .map(|line| line.to_string())
            .collect();
        let rows: Vec<Vec<Tile>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<Tile>, ParseTileError>>()
            })
            .collect::<Result<Vec<Vec<Tile>>, ParseTileError>>()
            .map_err(FieldError::ParseTileError)?;

        let height = rows.len();
        let width = rows.first().unwrap().len();
        if rows.iter().any(|row| row.len() != width) {
            return Err(FieldError::NotSquareField);
        }

        let tiles = rows.into_iter().flatten().collect();

        Ok(Field {
            tiles,
            width,
            height,
        })
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, tile) in self.tiles.iter().enumerate() {
            if i % self.width == 0 {
                f.write_str("\n")?;
            }

            let symbol = match *tile {
                Tile::VerticalPipe => "│",
                Tile::HorizontalPipe => "─",
                Tile::NorthEastBend => "╰",
                Tile::NorthWestBend => "╯",
                Tile::SouthWestBend => "╮",
                Tile::SouthEastBend => "╭",
                Tile::Ground => ".",
                Tile::StartingPosition => "S",
            };

            f.write_str(symbol)?;
        }

        Ok(())
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(Debug, Clone)]
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
    let mut field: Field = input.parse().unwrap();
    log::debug!("Field:\n{}", field);

    let pipe_loop = find_loop(&field);
    log::trace!("Loop: {:?}", pipe_loop);

    log::trace!(
        "Directions: {:?}",
        pipe_loop
            .iter()
            .map(|s| s.direction)
            .collect::<Vec<Direction>>()
    );

    let farthest_distance = pipe_loop.iter().map(|s| s.distance).max().unwrap();
    println!("{}", farthest_distance);

    field.filter(
        &pipe_loop
            .iter()
            .map(|s| s.position)
            .collect::<Vec<Position>>(),
    );
    log::debug!("Pipe:\n{}", field);

    let enclosed_tiles = find_enclosed_tiles(&field, &pipe_loop);
    let number_of_enclosed_tiles = enclosed_tiles.len();
    println!("{}", number_of_enclosed_tiles);
}

fn reverse_direction(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
        Direction::East => Direction::West,
    }
}

fn find_loop(field: &Field) -> Vec<State> {
    let start = field.get_start_position();

    let (start_1, start_2) = find_connected_pipes(field, &start).unwrap();
    log::trace!("{:?}, {:?}", start_1, start_2);
    let start_direction = reverse_direction(start_2.direction);

    let mut steps_1 = Vec::new();
    let mut steps_2 = Vec::new();

    let mut current_1 = start_1;
    let mut current_2 = start_2;

    while current_1.position != current_2.position {
        steps_1.push(current_1.clone());
        steps_2.push(current_2.clone());

        current_1 = get_next_state(field, current_1);
        current_2 = get_next_state(field, current_2);
    }

    steps_1.push(current_1);

    [State {
        position: start,
        direction: start_direction,
        distance: 0,
    }]
    .into_iter()
    .chain(
        steps_1
            .into_iter()
            .chain(steps_2.into_iter().rev().map(|s| State {
                direction: reverse_direction(
                    traverse_pipe(field.get(s.position).unwrap(), s.direction).unwrap(),
                ),
                ..s
            })),
    )
    .collect()
}

fn get_next_state(field: &Field, state: State) -> State {
    let current_tile = field.get(state.position).unwrap();
    let next_direction = traverse_pipe(current_tile, state.direction).unwrap();
    let next_position = get_position(state.position, next_direction).unwrap();

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
            get_position(*position, direction_1),
            get_position(*position, direction_2),
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

fn get_position(position: Position, direction: Direction) -> Option<Position> {
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

#[derive(Debug, Eq, PartialEq)]
enum SideOfLoop {
    Left,
    Right,
}

fn find_enclosed_tiles(field: &Field, pipe_loop: &[State]) -> HashSet<Position> {
    let mut loop_iter = pipe_loop
        .iter()
        .map(|s| (s.position, field.get(s.position).unwrap(), s.direction));

    // Is left will go from the perspective of the iteration;
    let mut known_inside: Option<SideOfLoop> = None;

    let mut left_ground_tiles = HashSet::new();
    let mut right_ground_tiles = HashSet::new();

    while let Some((current_position, current_tile, current_direction)) = loop_iter.next() {
        let (left_positions_to_check, right_positions_to_check) =
            get_positions_to_check(current_position, current_tile, current_direction);

        if known_inside != Some(SideOfLoop::Right) {
            for maybe_position in left_positions_to_check {
                if maybe_position.is_none() {
                    known_inside = Some(SideOfLoop::Right);
                    break;
                }

                let search_position = maybe_position.unwrap();
                let search_tile = field.get(search_position).unwrap();

                if search_tile == Tile::Ground {
                    if let Some(connected_grounds) = find_connected_grounds(field, search_position)
                    {
                        left_ground_tiles = left_ground_tiles
                            .union(&connected_grounds)
                            .cloned()
                            .collect();
                    } else {
                        known_inside = Some(SideOfLoop::Right);
                        break;
                    }
                }
            }
        }

        if known_inside != Some(SideOfLoop::Left) {
            for maybe_position in right_positions_to_check {
                if maybe_position.is_none() {
                    known_inside = Some(SideOfLoop::Left);
                    break;
                }

                let search_position = maybe_position.unwrap();
                let search_tile = field.get(search_position).unwrap();

                if let Tile::Ground = search_tile {
                    if let Some(connected_grounds) = find_connected_grounds(field, search_position)
                    {
                        right_ground_tiles = right_ground_tiles
                            .union(&connected_grounds)
                            .cloned()
                            .collect();
                    } else {
                        known_inside = Some(SideOfLoop::Left);
                        break;
                    }
                }
            }
        }
    }

    log::trace!("Left tiles: {:?}", left_ground_tiles);
    log::trace!("Right tiles: {:?}", right_ground_tiles);

    match known_inside.unwrap() {
        SideOfLoop::Left => left_ground_tiles,
        SideOfLoop::Right => right_ground_tiles,
    }
}

fn find_connected_grounds(field: &Field, position: Position) -> Option<HashSet<Position>> {
    let mut found = HashSet::new();
    let mut stack = Vec::new();

    stack.push(position);

    while let Some(search_position) = stack.pop() {
        found.insert(search_position);

        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if let Some(new_position) = get_position(search_position, direction) {
                let new_tile = field.get(new_position)?;
                if new_tile == Tile::Ground && !found.contains(&new_position) {
                    stack.push(new_position);
                }
            } else {
                return None;
            }
        }
    }

    Some(found)
}

fn get_positions_to_check(
    current_position: Position,
    current_tile: Tile,
    current_direction: Direction,
) -> (Vec<Option<Position>>, Vec<Option<Position>>) {
    let (left_tiles_to_check, right_tiles_to_check) = match (current_direction, current_tile) {
        (Direction::North, Tile::VerticalPipe) => {
            (vec![vec![Direction::West]], vec![vec![Direction::East]])
        }
        (Direction::North, Tile::SouthWestBend) => (
            vec![],
            vec![
                vec![Direction::East],
                vec![Direction::North],
                vec![Direction::North, Direction::East],
            ],
        ),
        (Direction::North, Tile::SouthEastBend) => (
            vec![
                vec![Direction::West],
                vec![Direction::North],
                vec![Direction::North, Direction::West],
            ],
            vec![],
        ),
        (Direction::South, Tile::VerticalPipe) => {
            (vec![vec![Direction::East]], vec![vec![Direction::West]])
        }
        (Direction::South, Tile::NorthEastBend) => (
            vec![],
            vec![
                vec![Direction::West],
                vec![Direction::South],
                vec![Direction::South, Direction::West],
            ],
        ),
        (Direction::South, Tile::NorthWestBend) => (
            vec![
                vec![Direction::East],
                vec![Direction::South],
                vec![Direction::South, Direction::East],
            ],
            vec![],
        ),
        (Direction::West, Tile::HorizontalPipe) => {
            (vec![vec![Direction::South]], vec![vec![Direction::North]])
        }
        (Direction::West, Tile::NorthEastBend) => (
            vec![
                vec![Direction::West],
                vec![Direction::South],
                vec![Direction::South, Direction::West],
            ],
            vec![],
        ),
        (Direction::West, Tile::SouthEastBend) => (
            vec![],
            vec![
                vec![Direction::West],
                vec![Direction::North],
                vec![Direction::North, Direction::West],
            ],
        ),
        (Direction::East, Tile::HorizontalPipe) => {
            (vec![vec![Direction::North]], vec![vec![Direction::South]])
        }
        (Direction::East, Tile::NorthWestBend) => (
            vec![],
            vec![
                vec![Direction::East],
                vec![Direction::South],
                vec![Direction::South, Direction::East],
            ],
        ),
        (Direction::East, Tile::SouthWestBend) => (
            vec![
                vec![Direction::East],
                vec![Direction::North],
                vec![Direction::North, Direction::East],
            ],
            vec![],
        ),

        (_, _) => (vec![], vec![]),
    };

    let left_positions: Vec<Option<Position>> =
        move_directions(current_position, left_tiles_to_check);
    let right_positions: Vec<Option<Position>> =
        move_directions(current_position, right_tiles_to_check);

    (left_positions, right_positions)
}

fn move_directions(
    current_position: Position,
    directions: Vec<Vec<Direction>>,
) -> Vec<Option<Position>> {
    directions
        .into_iter()
        .map(|directions| {
            directions.into_iter().fold(
                Some(current_position),
                |maybe_position: Option<Position>, direction: Direction| {
                    maybe_position.map(|p| get_position(p, direction)).flatten()
                },
            )
        })
        .collect()
}
