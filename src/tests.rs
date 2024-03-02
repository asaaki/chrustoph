use super::*;

#[test]
fn test_instance() {
    let chrustoph = Chrustoph::new();
    assert_eq!(chrustoph, Chrustoph);
}

#[test]
fn test_name() {
    let chrustoph = Chrustoph::new();
    assert_eq!(chrustoph.name(), "Chrustoph");
}

#[test]
fn test_rusty_name() {
    let chrustoph = Chrustoph::new();
    assert_eq!(chrustoph.rusty_name(), "ChRustoph");
}

#[test]
fn test_real_name() {
    let chrustoph = Chrustoph::new();
    assert_eq!(chrustoph.real_name(), "Christoph");
}

#[test]
fn test_nick_name() {
    let chrustoph = Chrustoph::new();
    assert_eq!(chrustoph.nick_name(), "asaaki");
}

#[test]
fn test_equality_and_order() {
    assert!(Chrustoph == Chrustoph::new());
    assert!(Chrustoph::new() == Chrustoph);

    assert!(Chrustoph <= Chrustoph::new()); // technically semi-false ;-)
    assert!(Chrustoph::new() >= Chrustoph); // technically semi-false ;-)

    assert_eq!(Chrustoph < Chrustoph::new(), false);
    assert_eq!(Chrustoph::new() > Chrustoph, false);

    let mut ccc = vec![Chrustoph::new(), Chrustoph, Chrustoph::default()];
    ccc.sort();
    assert_eq!(ccc, vec![Chrustoph, Chrustoph, Chrustoph]);
}

#[test]
fn test_hash() {
    use std::hash::{DefaultHasher, Hash, Hasher};

    let chrustoph = Chrustoph::new();
    let mut hasher = DefaultHasher::new();
    chrustoph.hash(&mut hasher);
    assert_ne!(hasher.finish(), 0);
}

#[test]
fn test_debug() {
    let chrustoph = Chrustoph::new();
    assert_eq!(format!("{:?}", chrustoph), "Chrustoph");
}

#[test]
fn test_clone() {
    let chrustoph = Chrustoph::new();
    let chrustoph_clone = chrustoph.clone();
    assert_eq!(chrustoph, chrustoph_clone);
}

#[test]
fn test_copy() {
    let chrustoph = Chrustoph::new();
    let chrustoph_copy = chrustoph;
    assert_eq!(chrustoph, chrustoph_copy);
}
