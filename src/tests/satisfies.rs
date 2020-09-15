use crate::*;

#[test]
fn basic_eq() {
    assert!(!satisfies("1.2.3", "=2.0.0").unwrap());
    assert!(!satisfies("1.2.3", "=1.2.0").unwrap());

    assert!(satisfies("1.2.3", "=1.2.3").unwrap());
}

#[test]
fn basic_caret() {
    assert!(!satisfies("1.2.3", "^2.0.0").unwrap());
    assert!(satisfies("1.2.3", "^1.2.0").unwrap());

    assert!(satisfies("1.2.3", "^1.2.3").unwrap());
}

#[test]
fn basic_tilde() {
    assert!(!satisfies("1.2.3", "~2.0.0").unwrap());
    assert!(satisfies("1.2.3", "~1.2.0").unwrap());

    assert!(!satisfies("1.3.0", "~1.2.3").unwrap());
}

#[test]
fn basic_wildcard() {
    assert!(satisfies("1.2.3", "*").unwrap());
    assert!(satisfies("1.2.3", "1.*").unwrap());
    assert!(satisfies("1.2.3", "1.2.*").unwrap());
}

#[test]
fn basic_gt() {
    assert!(satisfies("2.0.0", ">1.2.3").unwrap());
    assert!(satisfies("1.3.0", ">1.2.3").unwrap());
    assert!(satisfies("1.2.4", ">1.2.3").unwrap());

    assert!(!satisfies("0.2.3", ">1.2.3").unwrap());
    assert!(!satisfies("1.1.3", ">1.2.3").unwrap());
    assert!(!satisfies("1.2.2", ">1.2.3").unwrap());

    assert!(!satisfies("1.2.3", ">1.2.3").unwrap());
}

#[test]
fn basic_lt() {
    assert!(satisfies("0.2.3", "<1.2.3").unwrap());
    assert!(satisfies("1.1.3", "<1.2.3").unwrap());
    assert!(satisfies("1.2.2", "<1.2.3").unwrap());

    assert!(!satisfies("1.2.3", "<1.2.3").unwrap());
    assert!(!satisfies("1.3.3", "<1.2.3").unwrap());
    assert!(!satisfies("1.2.4", "<1.2.3").unwrap());
}
