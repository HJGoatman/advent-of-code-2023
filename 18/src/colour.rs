use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, Clone, Copy)]
pub(super) enum Colour {
    RGB(u8, u8, u8),
}

impl FromStr for Colour {
    type Err = ParseColourError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = &s[2..s.len() - 1];

        const HEX_RADIX: u32 = 16;

        let red =
            u8::from_str_radix(&hex[0..2], HEX_RADIX).map_err(ParseColourError::InvalidRed)?;
        let green =
            u8::from_str_radix(&hex[2..4], HEX_RADIX).map_err(ParseColourError::InvalidGreen)?;
        let blue =
            u8::from_str_radix(&hex[4..6], HEX_RADIX).map_err(ParseColourError::InvalidBlue)?;

        Ok(Colour::RGB(red, green, blue))
    }
}

#[derive(Debug)]
pub(super) enum ParseColourError {
    InvalidRed(ParseIntError),
    InvalidGreen(ParseIntError),
    InvalidBlue(ParseIntError),
}
