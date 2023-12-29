use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub y: usize,
    pub x: usize,
}

pub type HeatLossAmount = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HeatLossMap {
    values: Vec<HeatLossAmount>,
    width: usize,
    height: usize,
}

impl HeatLossMap {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn get_index(&self, position: Position) -> usize {
        position.y * self.width + position.x
    }

    pub fn get(&self, position: Position) -> Option<HeatLossAmount> {
        if position.x >= self.width || position.y >= self.height {
            return None;
        }

        let lookup_index = self.get_index(position);

        Some(self.values[lookup_index])
    }
}

#[derive(Debug)]
pub enum ParseHeatLossMapError {
    ParseHeatLossAmountError(ParseIntError),
}

impl FromStr for HeatLossMap {
    type Err = ParseHeatLossMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = s
            .split('\n')
            .filter(|line| line != &"")
            .map(|line| line.to_string())
            .collect();
        let rows: Vec<Vec<HeatLossAmount>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse())
                    .collect::<Result<Vec<HeatLossAmount>, ParseIntError>>()
            })
            .collect::<Result<Vec<Vec<HeatLossAmount>>, ParseIntError>>()
            .map_err(ParseHeatLossMapError::ParseHeatLossAmountError)?;

        let height = rows.len();
        let width = rows.first().unwrap().len();

        let values = rows.into_iter().flatten().collect();

        Ok(HeatLossMap {
            values,
            width,
            height,
        })
    }
}

impl Display for HeatLossMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, amount) in self.values.iter().enumerate() {
            if i % self.width == 0 {
                f.write_str("\n")?;
            }

            f.write_str(&amount.to_string())?;
        }

        Ok(())
    }
}
