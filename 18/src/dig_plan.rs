use std::num::ParseIntError;
use std::str::FromStr;

use crate::colour::{Colour, ParseColourError};

#[derive(Debug)]
pub(super) struct DigPlan {
    pub(super) instructions: Vec<DigInstruction>,
}

impl FromStr for DigPlan {
    type Err = ParseDigPlanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .split('\n')
            .map(|line| line.parse())
            .collect::<Result<Vec<DigInstruction>, ParseDigInstructionError>>()
            .map_err(ParseDigPlanError::ParseDigInstructionError)?;
        Ok(DigPlan { instructions })
    }
}

#[derive(Debug)]
pub(super) enum ParseDigPlanError {
    ParseDigInstructionError(ParseDigInstructionError),
}

#[derive(Debug, Clone, Copy)]
pub(super) struct DigInstruction {
    pub(super) direction: Direction,
    pub(super) amount: DigAmount,
    pub(super) colour: Colour,
}

impl FromStr for DigInstruction {
    type Err = ParseDigInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: [&str; 3] = s
            .split(' ')
            .collect::<Vec<&str>>()
            .try_into()
            .map_err(|_| ParseDigInstructionError::InvalidDigInstructionFormat)?;

        let direction = split[0]
            .parse()
            .map_err(ParseDigInstructionError::InvalidDirection)?;
        let amount = split[1]
            .parse()
            .map_err(ParseDigInstructionError::InvalidDigAmount)?;
        let colour = split[2]
            .parse()
            .map_err(ParseDigInstructionError::InvalidColour)?;

        Ok(DigInstruction {
            direction,
            amount,
            colour,
        })
    }
}

#[derive(Debug)]
pub(super) enum ParseDigInstructionError {
    InvalidDigInstructionFormat,
    InvalidDirection(ParseDirectionError),
    InvalidDigAmount(ParseIntError),
    InvalidColour(ParseColourError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(ParseDirectionError::UnknownDirection),
        }
    }
}

#[derive(Debug)]
pub(super) enum ParseDirectionError {
    UnknownDirection,
}

type DigAmount = u64;
