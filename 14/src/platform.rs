use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

#[derive(Debug, Clone, Copy)]
pub enum TiltDirection {
    North,
    West,
    South,
    East,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TiltResult {
    RocksMoved,
    NothingMoved,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

        let height = self.height;
        let width = self.width;

        let position_iter: Box<dyn Iterator<Item = Position>> = match tilt_direction {
            TiltDirection::North => {
                Box::new((0..height).flat_map(|y| (0..width).map(move |x| Position { x, y })))
            }
            TiltDirection::West => {
                Box::new((0..width).flat_map(|x| (0..height).map(move |y| Position { x, y })))
            }
            TiltDirection::South => Box::new(
                (0..height)
                    .rev()
                    .flat_map(|y| (0..width).rev().map(move |x| Position { x, y })),
            ),
            TiltDirection::East => Box::new(
                (0..width)
                    .rev()
                    .flat_map(|x| (0..height).rev().map(move |y| Position { x, y })),
            ),
        };

        for position in position_iter {
            if let Some(adjacent_position) = get_adjacent_position(position, tilt_direction) {
                if let (Some(space), Some(space_below)) =
                    (self.get(position), self.get(adjacent_position))
                {
                    if space == Space::EmptySpace && space_below == Space::RoundedRock {
                        self.swap(position, adjacent_position);
                        tilt_result = TiltResult::RocksMoved;
                    }
                }
            }
        }

        tilt_result
    }

    pub fn spin_cycle(&mut self) {
        for direction in &[
            TiltDirection::North,
            TiltDirection::West,
            TiltDirection::South,
            TiltDirection::East,
        ] {
            while self.tilt(*direction) == TiltResult::RocksMoved {}
            log::trace!("{:?}", direction);
            log::trace!("{}", self);
        }
    }
}

fn get_adjacent_position(position: Position, tilt_direction: TiltDirection) -> Option<Position> {
    match tilt_direction {
        TiltDirection::North => Some(Position {
            x: position.x,
            y: position.y + 1,
        }),
        TiltDirection::East => {
            if position.x == 0 {
                return None;
            }

            Some(Position {
                x: position.x - 1,
                y: position.y,
            })
        }
        TiltDirection::South => {
            if position.y == 0 {
                return None;
            }

            Some(Position {
                x: position.x,
                y: position.y - 1,
            })
        }
        TiltDirection::West => Some(Position {
            x: position.x + 1,
            y: position.y,
        }),
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
