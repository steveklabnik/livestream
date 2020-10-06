#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::error;

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
    ExpectedWildcard,
    InvalidPrerelease,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl error::Error for ParseError {}

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

    let (_prerelease, rest) = parse_prerelease(rest)?;

    if !rest.is_empty() {
        return Err(ParseError::ExtraInput);
    }

    Ok(Version {
        major,
        minor,
        patch,
    })
}

// A pre-release version MAY be denoted by appending a hyphen and a series of
// dot separated identifiers immediately following the patch version.
//
// Identifiers MUST comprise only ASCII alphanumerics and hyphens [0-9A-Za-z-].
//
// Identifiers MUST NOT be empty. Numeric identifiers MUST NOT include leading
// zeroes. Pre-release versions have a lower precedence than the associated
// normal version. A pre-release version indicates that the version is unstable
// and might not satisfy the intended compatibility requirements as denoted by
// its associated normal version.
//
// Examples: 1.0.0-alpha, 1.0.0-alpha.1,
// 1.0.0-0.3.7, 1.0.0-x.7.z.92, 1.0.0-x-y-z.–.
fn parse_prerelease(input: &str) -> Result<(Option<&str>, &str), ParseError> {
    if !input.starts_with('-') {
        return Ok((None, input));
    }

    // skip the -
    let mut pos = 1;
    let mut start_of_identifier = true;
    let mut identifier_pos = -1;
    let mut starts_with_zero = false;
    let mut all_numbers = true;

    // check for non-empty initial identifier
    if input[1..].len() == 0 {
        return Err(ParseError::InvalidPrerelease);
    }

    for byte in input[1..].bytes() {
        // is the character valid at all?
        if !(matches!(byte, b'0'..=b'9' | b'A'..=b'Z' | b'a'..=b'z' | b'-' | b'.')) {
            return Err(ParseError::InvalidPrerelease);
        }

        identifier_pos += 1;

        if byte == b'.' {
            identifier_pos = -1;
            starts_with_zero = false;
            all_numbers = true;
        } else if !matches!(byte, b'0'..=b'9') {
            all_numbers = false;
        }

        // if we're at the start of an identifier, we need to see if it is zero
        // in order to check for leading zeroes later.
        if start_of_identifier {
            start_of_identifier = false;

            if byte == b'0' {
                starts_with_zero = true;
            }

            if byte == b'.' {
                return Err(ParseError::InvalidPrerelease);
            }
        }

        // if we are a ., then we're gonna be at the start of an identifier on
        // the next iteration.
        if byte == b'.' {
            start_of_identifier = true;
        }

        pos += 1;
    }

    if start_of_identifier {
        return Err(ParseError::InvalidPrerelease);
    }

    if starts_with_zero && identifier_pos > 0 && all_numbers {
        return Err(ParseError::InvalidPrerelease);
    }

    return Ok((None, &input[pos..]));
}

#[derive(Debug, PartialEq)]
enum Operation {
    Caret,
    Eq,
    Tilde,
    Wildcard,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
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
    let (op, range) = parse_op(range).unwrap_or((Operation::Caret, range));

    if let Operation::Wildcard = op {
        return Ok(true);
    }

    let (version_major, version) = parse_number(version)?;
    let version = parse_dot(version)?;

    let (range_major, range) = match parse_number(range) {
        Ok(result) => result,
        Err(_) => {
            return Ok(parse_wildcard(range).is_ok());
        }
    };

    let range = parse_dot(range)?;

    match op {
        Operation::Eq => {
            if version_major != range_major {
                return Ok(false);
            }
        }
        Operation::Caret => {
            if version_major != range_major {
                return Ok(false);
            }
        }
        Operation::Tilde => {
            if version_major != range_major {
                return Ok(false);
            }
        }
        Operation::GreaterThan => {
            if version_major > range_major {
                return Ok(true);
            }
        }
        Operation::GreaterThanEqual => {
            if version_major >= range_major {
                return Ok(true);
            }
        }
        Operation::LessThan => {
            if version_major < range_major {
                return Ok(true);
            }
        }
        Operation::LessThanEqual => {
            if version_major <= range_major {
                return Ok(true);
            }
        }
        Operation::Wildcard => {
            unreachable!();
        }
    }

    let (version_minor, version) = parse_number(version)?;
    let version = parse_dot(version)?;

    let (range_minor, range) = match parse_number(range) {
        Ok(result) => result,
        Err(_) => {
            return Ok(parse_wildcard(range).is_ok());
        }
    };
    let range = parse_dot(range)?;

    match op {
        Operation::Eq => {
            if version_minor != range_minor {
                return Ok(false);
            }
        }
        Operation::Caret => {
            if version_minor < range_minor {
                return Ok(false);
            }
        }
        Operation::Tilde => {
            if version_minor != range_minor {
                return Ok(false);
            }
        }
        Operation::GreaterThan => {
            if version_minor > range_minor {
                return Ok(true);
            }
        }
        Operation::GreaterThanEqual => {
            if version_minor >= range_minor {
                return Ok(true);
            }
        }
        Operation::LessThan => {
            if version_minor < range_minor {
                return Ok(true);
            }
        }
        Operation::LessThanEqual => {
            if version_minor <= range_minor {
                return Ok(true);
            }
        }
        Operation::Wildcard => {
            unreachable!();
        }
    }

    let (version_patch, _version) = parse_number(version)?;
    let (range_patch, _range) = match parse_number(range) {
        Ok(result) => result,
        Err(_) => {
            return Ok(parse_wildcard(range).is_ok());
        }
    };

    match op {
        Operation::Eq => {
            if version_patch != range_patch {
                return Ok(false);
            }
        }
        Operation::Caret => {
            if version_patch < range_patch {
                return Ok(false);
            }
        }
        Operation::Tilde => {
            if version_patch < range_patch {
                return Ok(false);
            }
        }
        Operation::GreaterThan => {
            if version_patch <= range_patch {
                return Ok(false);
            }
        }
        Operation::GreaterThanEqual => {
            if version_patch < range_patch {
                return Ok(false);
            }
        }
        Operation::LessThan => {
            if version_patch >= range_patch {
                return Ok(false);
            }
        }
        Operation::LessThanEqual => {
            if version_patch > range_patch {
                return Ok(false);
            }
        }
        Operation::Wildcard => {
            unreachable!();
        }
    }

    Ok(true)
}

fn parse_op(range: &str) -> Option<(Operation, &str)> {
    if range.starts_with("^") {
        Some((Operation::Caret, &range[1..]))
    } else if range.starts_with("=") {
        Some((Operation::Eq, &range[1..]))
    } else if range.starts_with("~") {
        Some((Operation::Tilde, &range[1..]))
    } else if range.starts_with("*") {
        Some((Operation::Wildcard, &range[1..]))
    } else if range.starts_with(">") {
        if range.starts_with(">=") {
            Some((Operation::GreaterThanEqual, &range[2..]))
        } else {
            Some((Operation::GreaterThan, &range[1..]))
        }
    } else if range.starts_with("<") {
        if range.starts_with("<=") {
            Some((Operation::LessThanEqual, &range[2..]))
        } else {
            Some((Operation::LessThan, &range[1..]))
        }
    } else {
        None
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

fn parse_wildcard(rest: &str) -> Result<&str, ParseError> {
    if rest.starts_with("*") {
        return Ok(&rest[1..]);
    }

    Err(ParseError::ExpectedWildcard)
}

#[cfg(test)]
mod tests {
    mod parse;
    mod satisfies;
}
