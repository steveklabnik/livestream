#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::error::Error;

use core::fmt;

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

//type Result<T> = core::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NotAVersion,
    ExtraInput,
    ExpectedDot,
    LeadingZero,
    EmptyString,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl Error for ParseError {}

pub fn parse(input: &str) -> Result<Version, ParseError> {
    if input.is_empty() {
        return Err(ParseError::EmptyString);
    }

    let rest = input;

    let (major, rest) = parse_number(rest)?;

    let rest = parse_dot(rest)?;

    let (minor, rest) = parse_number(rest)?;

    let rest = parse_dot(rest)?;

    let (patch, rest) = parse_number(rest)?;

    if !rest.is_empty() {
        return Err(ParseError::ExtraInput);
    }

    Ok(Version {
        major,
        minor,
        patch,
    })
}

#[derive(PartialEq)]
enum Operation {
    Caret,
    Eq,
}

#[derive(Debug)]
pub enum ParseRangeError {
    MalformedOperation,
}

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    ParseRangeError(ParseRangeError),
}

impl From<ParseRangeError> for Error {
    fn from(error: ParseRangeError) -> Error {
        Error::ParseRangeError(error)
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Error {
        Error::ParseError(error)
    }
}

pub fn satisfies(version: &str, range: &str) -> Result<bool, Error> {
    let (op, range) = parse_op(range)?;

    let (version_major, version) = parse_number(version)?;
    let version = parse_dot(version)?;

    let (range_major, range) = parse_number(range)?;
    let range = parse_dot(range)?;

    match op {
        Operation::Eq => {
            if version_major != range_major {
                return Ok(false);
            }
        },
        Operation::Caret => {
            if version_major != range_major {
                return Ok(false);
            }
        }
    }

    let (version_minor, version) = parse_number(version)?;
    let version = parse_dot(version)?;

    let (range_minor, range) = parse_number(range)?;
    let range = parse_dot(range)?;

    match op {
        Operation::Eq => {
            if version_minor != range_minor {
                return Ok(false);
            }
        },
        Operation::Caret => {
            if version_minor < range_minor {
                return Ok(false);
            }
        }
    }

    let (version_patch, _version) = parse_number(version)?;
    let (range_patch, _range) = parse_number(range)?;

    match op {
        Operation::Eq => {
            if version_patch != range_patch {
                return Ok(false);
            }
        },
        Operation::Caret => {
            if version_patch < range_patch {
                return Ok(false);
            }
        }
    }

    Ok(true)
}

fn parse_op(range: &str) -> Result<(Operation, &str), ParseRangeError> {
    if range.starts_with("^") {
        Ok((Operation::Caret, &range[1..]))
    } else if range.starts_with("=") {
        Ok((Operation::Eq, &range[1..]))
    } else {
        Err(ParseRangeError::MalformedOperation)
    }
}

fn parse_number(rest: &str) -> Result<(u64, &str), ParseError> {
    let mut non_digit_index = rest.len();

    for (index, c) in rest.bytes().enumerate() {
        if !matches!(c, b'0'..=b'9') {
            non_digit_index = index;
            break;
        }
    }

    if rest.starts_with("0") && non_digit_index > 1 {
        return Err(ParseError::LeadingZero);
    }

    let num = rest[..non_digit_index]
        .parse()
        .or(Err(ParseError::NotAVersion))?;

    Ok((num, &rest[non_digit_index..]))
}

fn parse_dot(rest: &str) -> Result<&str, ParseError> {
    if rest.starts_with(".") {
        return Ok(&rest[1..]);
    }

    Err(ParseError::ExpectedDot)
}

#[cfg(test)]
mod tests {
    mod parse;
    mod satisfies;
}
