use crate::colour::Colour;
use crate::dig_plan::{DigInstruction, DigPlan, Direction};

use std::fmt::Display;

use colored::Colorize;

#[derive(Debug)]
pub(super) struct Edge {
    pub(super) start: Position,
    pub(super) end: Position,
    pub(super) length: u64,
    pub(super) direction: Direction,
    pub(super) colour: Colour,
}

#[derive(Debug)]
pub(super) struct Trench {
    pub(super) edges: Vec<Edge>,
}

impl From<&DigPlan> for Trench {
    fn from(dig_plan: &DigPlan) -> Self {
        let mut edges = Vec::new();

        const START: Position = Position { y: 0, x: 0 };

        let mut current_position = START;

        for DigInstruction {
            direction,
            amount,
            colour,
        } in dig_plan.instructions.iter().copied()
        {
            let start = current_position;
            let end = move_direction(start, direction, amount);
            let length = ((start.x - end.x).abs() + (start.y - end.y).abs()) as u64;

            let edge = Edge {
                start,
                end,
                length,
                direction,
                colour,
            };

            edges.push(edge);

            current_position = end;
        }

        Trench { edges }
    }
}

pub fn move_direction(position: Position, direction: Direction, amount: u64) -> Position {
    let amount = amount as i64;

    match direction {
        Direction::Up => Position {
            y: position.y - amount,
            x: position.x,
        },
        Direction::Down => Position {
            y: position.y + amount,
            x: position.x,
        },
        Direction::Left => Position {
            y: position.y,
            x: position.x - amount,
        },
        Direction::Right => Position {
            y: position.y,
            x: position.x + amount,
        },
    }
}

impl Display for Trench {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (xs, ys): (Vec<i64>, Vec<i64>) = self
            .edges
            .iter()
            .map(|edge| edge.start)
            .map(|Position { x, y }| (x, y))
            .unzip();

        let min_x = *xs.iter().min().ok_or(std::fmt::Error)?;
        let max_x = *xs.iter().max().ok_or(std::fmt::Error)?;

        let min_y = *ys.iter().min().ok_or(std::fmt::Error)?;
        let max_y = *ys.iter().max().ok_or(std::fmt::Error)?;

        for y in min_y..max_y + 1 {
            f.write_str("\n")?;

            for x in min_x..max_x + 1 {
                let edge_search = self.edges.iter().find(|edge| {
                    let start = edge.start;
                    let end = edge.end;

                    match edge.direction {
                        Direction::Up => start.x == x && (end.y..start.y + 1).contains(&y),
                        Direction::Down => start.x == x && (start.y..end.y + 1).contains(&y),
                        Direction::Left => start.y == y && (end.x..start.x + 1).contains(&x),
                        Direction::Right => start.y == y && (start.x..end.x + 1).contains(&x),
                    }
                });

                if let Some(edge) = edge_search {
                    const DUG_OUT_PART: &str = "██";

                    if (edge.start == Position { x, y }) || (edge.end == Position { x, y }) {
                        f.write_str(DUG_OUT_PART)?;
                    } else {
                        let Colour::RGB(red, green, blue) = edge.colour;

                        f.write_fmt(format_args!(
                            "{}",
                            &DUG_OUT_PART.truecolor(red, green, blue)
                        ))?;
                    }
                } else {
                    f.write_str("  ")?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub(super) struct Position {
    pub(super) y: i64,
    pub(super) x: i64,
}
