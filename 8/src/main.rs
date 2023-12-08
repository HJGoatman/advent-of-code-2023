use std::collections::HashMap;
use std::env;
use std::fs;
use std::mem::swap;
use std::str::FromStr;
use std::usize;

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct ParseInstructionError;

impl TryFrom<char> for Instruction {
    type Error = ParseInstructionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(ParseInstructionError),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct NodeId {
    value: String,
}

#[derive(Debug)]
struct Node {
    id: NodeId,
    left: NodeId,
    right: NodeId,
}

#[derive(Debug)]
struct ParseNodeError;

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(" = ");

        let id_value = split.next().ok_or(ParseNodeError)?.to_string();
        let id = NodeId { value: id_value };

        let nodes = split.next().ok_or(ParseNodeError)?;

        let mut nodes_split = nodes[1..nodes.len() - 1].split(", ");

        let left_value = nodes_split.next().ok_or(ParseNodeError)?.to_string();
        let left = NodeId { value: left_value };

        let right_value = nodes_split.next().ok_or(ParseNodeError)?.to_string();

        let right = NodeId { value: right_value };

        Ok(Node { id, left, right })
    }
}

#[derive(Debug)]
struct Network {
    nodes: HashMap<NodeId, Node>,
}

impl Network {
    fn go_right(&self, current: &NodeId) -> Option<NodeId> {
        let current_node = self.nodes.get(current)?;

        Some(current_node.right.clone())
    }

    fn go_left(&self, current: &NodeId) -> Option<NodeId> {
        let current_node = self.nodes.get(current)?;

        Some(current_node.left.clone())
    }

    fn contains(&self, node: &NodeId) -> bool {
        self.nodes.contains_key(node)
    }
}

#[derive(Debug)]
enum ParseNetworkError {
    ParseNodeError(ParseNodeError),
}

impl FromStr for Network {
    type Err = ParseNetworkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nodes = s
            .split('\n')
            .map(|node_str| node_str.parse())
            .map(|res: Result<Node, ParseNodeError>| res.map(|node: Node| (node.id.clone(), node)))
            .collect::<Result<HashMap<NodeId, Node>, ParseNodeError>>()
            .map_err(ParseNetworkError::ParseNodeError)?;

        Ok(Network { nodes })
    }
}

fn load_input() -> String {
    let args: Vec<String> = env::args().collect();
    fs::read_to_string(args.get(1).unwrap()).expect("Should have been able to read the file")
}

fn main() {
    env_logger::init();

    let input = load_input();
    let mut split = input.split("\n\n");

    let instructions: Vec<Instruction> = split
        .next()
        .unwrap()
        .chars()
        .map(|c| Instruction::try_from(c).unwrap())
        .collect();
    log::debug!("Instructions: {:?}", instructions);

    let network: Network = split.next().unwrap().parse().unwrap();
    log::debug!("Network: {:?}", network);

    let start = NodeId {
        value: "AAA".to_string(),
    };
    let end = NodeId {
        value: "ZZZ".to_string(),
    };

    if network.contains(&start) && network.contains(&end) {
        let steps = traverse_network(&start, &end, &instructions, &network);

        println!("{}", steps);
    }

    let start_node_ids: Vec<NodeId> = network
        .nodes
        .keys()
        .filter(|node_id| node_id.value.ends_with('A'))
        .cloned()
        .collect();
    log::debug!("Start Nodes: {:?}", start_node_ids);

    let end_node_ids: Vec<NodeId> = network
        .nodes
        .keys()
        .filter(|node_id| node_id.value.ends_with('Z'))
        .cloned()
        .collect();
    log::debug!("End Nodes: {:?}", end_node_ids);

    let mut cycle_mapping: HashMap<(NodeId, NodeId), u32> = HashMap::new();
    for start_node in start_node_ids.iter() {
        for end_node in end_node_ids.iter() {
            let steps = find_network_cycle(start_node, end_node, &instructions, &network);

            if let Some(steps) = steps {
                cycle_mapping.insert((start_node.clone(), end_node.clone()), steps);
            }
        }
    }

    cycle_mapping.iter().for_each(|p| log::debug!("{:?}", p));

    let total: u64 = cycle_mapping
        .values()
        .cloned()
        .map(|v| v as u64)
        .fold(1, lcm);
    println!("{}", total);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        swap(&mut a, &mut b);
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    // LCM = a*b / gcd
    a * (b / gcd(a, b))
}

fn traverse_network(
    start_node: &NodeId,
    end_node: &NodeId,
    instructions: &[Instruction],
    network: &Network,
) -> u32 {
    let mut current_node = start_node.clone();
    let mut instructions_cycle = instructions.iter().cycle();

    let mut steps = 0;
    while &current_node != end_node {
        let next_instruction = instructions_cycle.next().unwrap();
        let next_node = match next_instruction {
            Instruction::Left => network.go_left(&current_node),
            Instruction::Right => network.go_right(&current_node),
        };

        current_node = next_node.unwrap();
        steps += 1;
    }

    steps
}

fn find_network_cycle(
    start_node: &NodeId,
    end_node: &NodeId,
    instructions: &[Instruction],
    network: &Network,
) -> Option<u32> {
    let mut current_node = start_node.clone();
    let mut instructions_cycle = instructions.iter().enumerate().cycle();
    let mut steps = 0;
    let mut previous_end_finds: Vec<u32> = Vec::new();
    let mut skip_first_detection = true;
    let no_cycle_threshold = instructions.len() as u32 * 90;

    loop {
        if steps > no_cycle_threshold {
            return None;
        }

        let (instruction_id, next_instruction) = instructions_cycle.next().unwrap();

        if current_node == *end_node {
            log::trace!("{}, {}", steps, instruction_id);

            if skip_first_detection {
                skip_first_detection = false;
                steps = 0;
            } else {
                const CYCLE_DETECTED_THRESHOLD: usize = 3;
                if previous_end_finds.len() == CYCLE_DETECTED_THRESHOLD {
                    return Some(steps);
                }

                let new_find = steps;

                if !previous_end_finds.is_empty() {
                    log::trace!("{:?}", previous_end_finds);
                    if previous_end_finds.iter().any(|find| *find != new_find) {
                        continue;
                    }
                }

                previous_end_finds.push(new_find);
                steps = 0;
            }
        }

        let next_node = match next_instruction {
            Instruction::Left => network.go_left(&current_node),
            Instruction::Right => network.go_right(&current_node),
        };

        current_node = next_node.unwrap();
        steps += 1;
    }
}
