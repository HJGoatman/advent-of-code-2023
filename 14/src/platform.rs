use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Space {
    RoundedRock,
    CubeShapedRock,
    EmptySpace,
}

#[derive(Debug)]
pub enum ParseSpaceError {
    Unknown,
}

impl TryFrom<char> for Space {
    type Error = ParseSpaceError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Space::RoundedRock),
            '#' => Ok(Space::CubeShapedRock),
            '.' => Ok(Space::EmptySpace),
            _ => Err(ParseSpaceError::Unknown),
        }
    }
}

pub enum TiltDirection {
    North,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TiltResult {
    RocksMoved,
    NothingMoved,
}

#[derive(Debug)]
pub struct Platform {
    spaces: Vec<Space>,
    width: usize,
    height: usize,
}

impl Platform {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn get_index(&self, position: Position) -> usize {
        position.y * self.width + position.x
    }

    pub fn get(&self, position: Position) -> Option<Space> {
        if position.x >= self.width || position.y >= self.height {
            return None;
        }

        let lookup_index = self.get_index(position);

        Some(self.spaces[lookup_index])
    }

    fn swap(&mut self, position_1: Position, position_2: Position) {
        let index_1 = self.get_index(position_1);
        let index_2 = self.get_index(position_2);

        self.spaces.swap(index_1, index_2);
    }

    pub fn tilt(&mut self, tilt_direction: TiltDirection) -> TiltResult {
        let mut tilt_result = TiltResult::NothingMoved;

        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position { x, y };
                let position_below = Position { x, y: y + 1 };

                if let (Some(space), Some(space_below)) =
                    (self.get(position), self.get(position_below))
                {
                    if space == Space::EmptySpace && space_below == Space::RoundedRock {
                        self.swap(position, position_below);
                        tilt_result = TiltResult::RocksMoved;
                    }
                }
            }
        }

        tilt_result
    }
}

#[derive(Debug)]
pub enum ParsePlatformError {
    ParseSpaceError(ParseSpaceError),
}

impl FromStr for Platform {
    type Err = ParsePlatformError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<String> = s
            .split('\n')
            .filter(|line| line != &"")
            .map(|line| line.to_string())
            .collect();
        let rows: Vec<Vec<Space>> = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(Space::try_from)
                    .collect::<Result<Vec<Space>, ParseSpaceError>>()
            })
            .collect::<Result<Vec<Vec<Space>>, ParseSpaceError>>()
            .map_err(ParsePlatformError::ParseSpaceError)?;

        let height = rows.len();
        let width = rows.first().unwrap().len();

        let spaces = rows.into_iter().flatten().collect();

        Ok(Platform {
            spaces,
            width,
            height,
        })
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, space) in self.spaces.iter().enumerate() {
            if i % self.width == 0 {
                f.write_str("\n")?;
            }

            let symbol = match *space {
                Space::RoundedRock => "O",
                Space::CubeShapedRock => "#",
                Space::EmptySpace => ".",
            };

            f.write_str(symbol)?;
        }

        Ok(())
    }
}
