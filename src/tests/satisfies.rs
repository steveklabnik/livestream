use crate::*;

#[test]
fn basic() {
    assert!(!satisfies("1.2.3", "=2.0.0").unwrap());
    assert!(!satisfies("1.2.3", "=1.2.0").unwrap());

    assert!(satisfies("1.2.3", "=1.2.3").unwrap());
}
