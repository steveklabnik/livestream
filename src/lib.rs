use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
}

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NotAVersion,
    ExtraInput,
    ExpectedDot,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for ParseError {}

pub fn parse(input: &str) -> Result<Version> {
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

fn parse_number(rest: &str) -> Result<(u64, &str)> {
    // figure out how many numbers we have

    let mut non_digit_index = rest.len();

    for (index, c) in rest.bytes().enumerate() {
        match c {
            b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9' => (),
            _ => {
                non_digit_index = index;
                break;
            }
        }
    }

    let num = rest[..non_digit_index]
        .parse()
        .or(Err(ParseError::NotAVersion))?;

    Ok((num, &rest[non_digit_index..]))
}

fn parse_dot(rest: &str) -> Result<&str> {
    if &rest[0..1] == "." {
        return Ok(&rest[1..]);
    }

    Err(ParseError::ExpectedDot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        assert!(parse("1.2.3").is_ok());

        assert_eq!(parse("1.2.3").unwrap().major, 1);
        assert_eq!(parse("1.2.3").unwrap().minor, 2);
        assert_eq!(parse("1.2.3").unwrap().patch, 3);

        assert_eq!(parse("11.22.33").unwrap().patch, 33);
    }

    #[test]
    fn no_extra_input() {
        assert_eq!(parse("1.2.3 "), Err(ParseError::ExtraInput));
    }

    #[test]
    fn couldnt_parse_version() {
        assert_eq!(parse(".2.3"), Err(ParseError::NotAVersion));
        assert_eq!(parse("1..3"), Err(ParseError::NotAVersion));
    }
}
