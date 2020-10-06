use crate::*;
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

#[test]
fn no_leading_zeroes() {
    assert_eq!(parse("01.2.3"), Err(ParseError::LeadingZero));
    assert_eq!(parse("00.2.3"), Err(ParseError::LeadingZero));
    assert!(parse("0.2.3").is_ok());
    assert!(parse("0.2.0").is_ok());
}

#[test]
fn empty_string() {
    assert_eq!(parse(""), Err(ParseError::EmptyString));
}

#[test]
fn basic_prerelease() {
    assert!(parse("1.2.3-pre").is_ok());

    assert!(parse("1.0.0-alpha").is_ok());
    assert!(parse("1.0.0-alpha.1").is_ok());
    assert!(parse("1.0.0-0.3.7").is_ok());
    assert!(parse("1.0.0-x.7.z.92").is_ok());
    assert!(parse("1.0.0-x-y-z.-").is_ok());
}

#[test]
fn prerelease_numeric_identifiers() {
    assert!(parse("1.2.3-beta.0").is_ok());
}

#[test]
fn prerelease_identifiers_must_not_be_empty() {
    assert!(parse("1.2.3-foo..").is_err());
    assert!(parse("1.2.3-").is_err());
    assert!(parse("1.2.3-foo.").is_err());
}
