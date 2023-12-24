use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub y: usize,
    pub x: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MirrorType {
    Forward,
    Backward,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SplitterType {
    Vertical,
    Horizontal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Tile {
    Mirror(MirrorType),
    Splitter(SplitterType),
    EmptySpace,
}

#[derive(Debug)]
pub enum ParseTileError {
    Unknown,
}

impl TryFrom<char> for Tile {
    type Error = ParseTileError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Tile::Splitter(SplitterType::Vertical)),
            '-' => Ok(Tile::Splitter(SplitterType::Horizontal)),
            '/' => Ok(Tile::Mirror(MirrorType::Forward)),
            '\\' => Ok(Tile::Mirror(MirrorType::Backward)),
            '.' => Ok(Tile::EmptySpace),
            _ => Err(ParseTileError::Unknown),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Contraption {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Contraption {
    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn get_index(&self, position: Position) -> usize {
        position.y * self.width + position.x
    }

    pub fn get(&self, position: Position) -> Option<Tile> {
        if position.x >= self.width || position.y >= self.height {
            return None;
        }

        let lookup_index = self.get_index(position);

        Some(self.tiles[lookup_index])
    }
}

#[derive(Debug)]
pub enum ParseContraptionError {
    ParseTileError(ParseTileError),
}

impl FromStr for Contraption {
    type Err = ParseContraptionError;

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
            .map_err(ParseContraptionError::ParseTileError)?;

        let height = rows.len();
        let width = rows.first().unwrap().len();

        let tiles = rows.into_iter().flatten().collect();

        Ok(Contraption {
            tiles,
            width,
            height,
        })
    }
}

impl Display for Contraption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, tile) in self.tiles.iter().enumerate() {
            if i % self.width == 0 {
                f.write_str("\n")?;
            }

            let symbol = match *tile {
                Tile::Mirror(MirrorType::Forward) => '/',
                Tile::Mirror(MirrorType::Backward) => '\\',
                Tile::Splitter(SplitterType::Horizontal) => '-',
                Tile::Splitter(SplitterType::Vertical) => '|',
                Tile::EmptySpace => '.',
            };

            f.write_str(&symbol.to_string())?;
        }

        Ok(())
    }
}
