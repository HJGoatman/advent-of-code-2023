mod heat_loss_map;

use heat_loss_map::HeatLossAmount;
use heat_loss_map::HeatLossMap;
use heat_loss_map::Position;

use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let heat_loss_map: HeatLossMap = input.parse().unwrap();
    log::debug!("{}", heat_loss_map);

    let lava_pool_position = Position { x: 0, y: 0 };
    let machine_parts_factory_position = Position {
        x: heat_loss_map.get_width() - 1,
        y: heat_loss_map.get_height() - 1,
    };

    const PART_1_MIN_BLOCKS_STRAIGHT: u8 = 0;
    const PART_1_MAX_BLOCKS_STRAIGHT: u8 = 4;

    let min_heat_loss = shortest_path(
        &heat_loss_map,
        lava_pool_position,
        machine_parts_factory_position,
        PART_1_MIN_BLOCKS_STRAIGHT,
        PART_1_MAX_BLOCKS_STRAIGHT,
    )
    .unwrap();

    println!("{}", min_heat_loss);

    const PART_2_MIN_BLOCKS_STRAIGHT: u8 = 4;
    const PART_2_MAX_BLOCKS_STRAIGHT: u8 = 11;

    let min_heat_loss = shortest_path(
        &heat_loss_map,
        lava_pool_position,
        machine_parts_factory_position,
        PART_2_MIN_BLOCKS_STRAIGHT,
        PART_2_MAX_BLOCKS_STRAIGHT,
    )
    .unwrap();

    println!("{}", min_heat_loss);
}

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

struct MinPriorityQueue<T> {
    binary_heap: BinaryHeap<Reverse<T>>,
}

impl<T: Ord> MinPriorityQueue<T> {
    fn new(n: usize) -> MinPriorityQueue<T> {
        let binary_heap = BinaryHeap::with_capacity(n);
        MinPriorityQueue { binary_heap }
    }

    fn push(&mut self, value: T) {
        self.binary_heap.push(Reverse(value))
    }

    fn pop(&mut self) -> Option<T> {
        let reversed = self.binary_heap.pop()?;

        Some(reversed.0)
    }
}

impl<T: Ord> FromIterator<T> for MinPriorityQueue<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let binary_heap = iter.into_iter().map(|value| Reverse(value)).collect();
        MinPriorityQueue { binary_heap }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    position: Position,
    total_heat_loss: HeatLossAmount,
    direction: Option<Direction>,
    distance_in_current_direction: u8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_heat_loss.cmp(&other.total_heat_loss)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for State {}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.position.eq(&other.position)
            && self.direction.eq(&other.direction)
            && self
                .distance_in_current_direction
                .eq(&other.distance_in_current_direction)
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.hash(state);
        self.distance_in_current_direction.hash(state);
    }
}

fn shortest_path(
    heat_loss_map: &HeatLossMap,
    start: Position,
    goal: Position,
    min_blocks_straight: u8,
    max_blocks_straight: u8,
) -> Option<HeatLossAmount> {
    let mut dist: HashMap<State, HeatLossAmount> = HashMap::new();
    let mut heap = MinPriorityQueue::new(dist.len());

    let initial_state = State {
        total_heat_loss: 0,
        position: start,
        direction: None,
        distance_in_current_direction: 0,
    };

    dist.insert(initial_state, 0);
    heap.push(initial_state);

    while let Some(state) = heap.pop() {
        if state.position == goal && (state.distance_in_current_direction >= min_blocks_straight) {
            return Some(state.total_heat_loss);
        }

        if state.total_heat_loss > dist[&state] {
            continue;
        }

        for next_state in get_next_states(
            heat_loss_map,
            state,
            min_blocks_straight,
            max_blocks_straight,
        ) {
            if let Some(existing_total_heat_loss) = dist.get(&next_state) {
                if next_state.total_heat_loss >= *existing_total_heat_loss {
                    continue;
                }
            }

            dist.insert(next_state, next_state.total_heat_loss);
            heap.push(next_state);
        }
    }

    None
}

fn get_next_states(
    heat_loss_map: &HeatLossMap,
    state: State,
    min_blocks_straight: u8,
    max_blocks_straight: u8,
) -> Vec<State> {
    let next_turns = match state.direction {
        Some(Direction::Up) => vec![
            (Direction::Left, false),
            (Direction::Up, true),
            (Direction::Right, false),
        ],
        Some(Direction::Left) => vec![
            (Direction::Down, false),
            (Direction::Left, true),
            (Direction::Up, false),
        ],
        Some(Direction::Down) => vec![
            (Direction::Right, false),
            (Direction::Down, true),
            (Direction::Left, false),
        ],
        Some(Direction::Right) => vec![
            (Direction::Down, false),
            (Direction::Right, true),
            (Direction::Up, false),
        ],
        None => vec![
            (Direction::Left, true),
            (Direction::Up, true),
            (Direction::Right, true),
            (Direction::Down, true),
        ],
    };

    let mut new_states = Vec::new();

    for (turn, is_straight) in next_turns {
        if !is_straight && (state.distance_in_current_direction < min_blocks_straight) {
            continue;
        }

        if let Some(position) = get_adjacent_position(state.position, turn) {
            if let Some(current_g_score) = heat_loss_map.get(position) {
                let total_heat_loss = state.total_heat_loss + current_g_score;

                let direction = Some(turn);
                let distance_in_current_direction = if state.direction.is_none() || is_straight {
                    state.distance_in_current_direction + 1
                } else {
                    1
                };

                if distance_in_current_direction >= max_blocks_straight {
                    continue;
                }

                let new_state = State {
                    position,
                    total_heat_loss,
                    direction,
                    distance_in_current_direction,
                };

                new_states.push(new_state);
            };
        }
    }

    new_states
}
