// Import the necessary modules

use crate::ParseError::{InvalidBadDeeds, InvalidGoodDeeds, NoBadDeeds, NoGoodDeeds, NoName};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Pointer};

pub enum ParseError {
    // 1. Add variants here (read description)
    NoName,
    NoGoodDeeds,
    NoBadDeeds,
    InvalidGoodDeeds,
    InvalidBadDeeds,
}

// 2. Implement the Error trait for ParseError

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParseError::NoName => {
                    "Name field is missing"
                }
                ParseError::NoGoodDeeds => {
                    "Good deeds field is missing"
                }
                ParseError::NoBadDeeds => {
                    "Bad deeds field is missing"
                }
                ParseError::InvalidGoodDeeds => {
                    "Good deeds value is invalid"
                }
                ParseError::InvalidBadDeeds => {
                    "Bad deeds value is invalid"
                }
            }
        )
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Error for ParseError {}

#[derive(Debug)]
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Kid { name, niceness }
    }

    pub fn parse_row(csv_row: &str) -> Result<Kid, ParseError> {
        // 3. Update the code to return meaningful errors
        let mut fields = csv_row.split(',');
        let name = fields.next().ok_or(NoName)?.to_string();
        if name == "" {
            return Err(NoName);
        }
        let good_deeds = fields.next().ok_or(NoGoodDeeds)?;

        if good_deeds == "" {
            return Err(NoGoodDeeds);
        }

        let good_deeds = good_deeds.parse::<u32>().map_err(|_| InvalidGoodDeeds)?;

        let bad_deeds = fields.next().ok_or(NoBadDeeds)?;
        if bad_deeds == "" {
            return Err(NoBadDeeds);
        }
        let bad_deeds = bad_deeds.parse::<u32>().map_err(|_| InvalidBadDeeds)?;

        Ok(Kid::new(name, good_deeds, bad_deeds))
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

fn main() {
    println!("{:?}", Kid::parse_row("Alice,,3"));
}
