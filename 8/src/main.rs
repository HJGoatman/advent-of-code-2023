use env_logger;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;

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

        let id_value = split.next().ok_or_else(|| ParseNodeError)?.to_string();
        let id = NodeId { value: id_value };

        let nodes = split.next().ok_or_else(|| ParseNodeError)?;

        let mut nodes_split = nodes[1..nodes.len() - 1].split(", ");

        let left_value = nodes_split
            .next()
            .ok_or_else(|| ParseNodeError)?
            .to_string();
        let left = NodeId { value: left_value };

        let right_value = nodes_split
            .next()
            .ok_or_else(|| ParseNodeError)?
            .to_string();

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
            .map_err(|v| ParseNetworkError::ParseNodeError(v))?;

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

    let mut current_node = start;
    let mut instructions_cycle = instructions.iter().cycle();
    let mut steps = 0;
    while current_node != end {
        let next_instruction = instructions_cycle.next().unwrap();

        let next_node = match next_instruction {
            Instruction::Left => network.go_left(&current_node),
            Instruction::Right => network.go_right(&current_node),
        };

        current_node = next_node.unwrap();
        steps += 1;
    }

    println!("{}", steps);
}
