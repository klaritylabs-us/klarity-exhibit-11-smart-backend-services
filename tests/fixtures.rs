use std::{fs, path::PathBuf};
use smart_backend::validate_jwt_file;

fn ex(n: &str) -> String {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("examples"); p.push(n);
    fs::read_to_string(p).unwrap()
}

#[test]
fn valid_fixture() {
    let s = ex("valid-jwt.txt");
    let r = validate_jwt_file(&s); assert!(r.is_valid());
}

#[test]
fn invalid_fixture() {
    let s = ex("invalid-jwt.txt");
    let r = validate_jwt_file(&s); assert!(!r.is_valid());
}
