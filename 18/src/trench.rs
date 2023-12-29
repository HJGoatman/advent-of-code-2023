use crate::colour::Colour;
use crate::dig_plan::{DigInstruction, DigPlan, Direction};

use std::collections::BTreeMap;
use std::fmt::Display;

use colored::Colorize;

#[derive(Debug)]
pub(super) enum TrenchPart {
    Edge(Colour),
    Interior,
}

#[derive(Debug)]
pub(super) struct Trench {
    pub(super) dug_out_positions: BTreeMap<Position, TrenchPart>,
}

impl From<DigPlan> for Trench {
    fn from(dig_plan: DigPlan) -> Self {
        let mut dug_out_positions = BTreeMap::new();

        const START: Position = Position { y: 0, x: 0 };

        dug_out_positions.insert(
            START,
            TrenchPart::Edge(dig_plan.instructions.first().unwrap().colour),
        );

        let mut current_position = START;

        for DigInstruction {
            direction,
            amount,
            colour,
        } in dig_plan.instructions
        {
            for _ in 0..amount {
                current_position = match direction {
                    Direction::Up => Position {
                        y: current_position.y - 1,
                        x: current_position.x,
                    },
                    Direction::Down => Position {
                        y: current_position.y + 1,
                        x: current_position.x,
                    },
                    Direction::Left => Position {
                        y: current_position.y,
                        x: current_position.x - 1,
                    },
                    Direction::Right => Position {
                        y: current_position.y,
                        x: current_position.x + 1,
                    },
                };

                dug_out_positions.insert(current_position, TrenchPart::Edge(colour));
            }
        }

        Trench { dug_out_positions }
    }
}

impl Display for Trench {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (xs, ys): (Vec<i32>, Vec<i32>) = self
            .dug_out_positions
            .iter()
            .map(|(Position { x, y }, _)| (x, y))
            .unzip();

        let min_x = *xs.iter().min().ok_or(std::fmt::Error)?;
        let max_x = *xs.iter().max().ok_or(std::fmt::Error)?;

        let min_y = *ys.iter().min().ok_or(std::fmt::Error)?;
        let max_y = *ys.iter().max().ok_or(std::fmt::Error)?;

        for y in min_y..max_y + 1 {
            f.write_str("\n")?;

            for x in min_x..max_x + 1 {
                let current_position = Position { y, x };

                if let Some(trench_part) = self.dug_out_positions.get(&current_position) {
                    const DUG_OUT_PART: &str = "██";

                    match trench_part {
                        TrenchPart::Edge(Colour::RGB(red, green, blue)) => {
                            f.write_fmt(format_args!(
                                "{}",
                                &DUG_OUT_PART.truecolor(*red, *green, *blue)
                            ))?;
                        }
                        TrenchPart::Interior => {
                            f.write_str(DUG_OUT_PART)?;
                        }
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
    pub(super) y: i32,
    pub(super) x: i32,
}
