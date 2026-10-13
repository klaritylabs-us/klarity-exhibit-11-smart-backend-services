use std::{env, fs, process::ExitCode};
use smart_backend::validate_jwt_file;

fn main() -> ExitCode {
    let path = env::args().nth(1).expect("usage: validate-smart-backend <file>");
    let text = fs::read_to_string(path).unwrap();
    let report = validate_jwt_file(&text);
    println!("{}", serde_json::to_string_pretty(&report.issues).unwrap());
    ExitCode::from(if report.is_valid() { 0 } else { 1 })
}
