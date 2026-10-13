use crate::outcome::{ValidationIssue, ValidationReport};

pub fn validate_jwt_file(text: &str) -> ValidationReport {
    let mut r = ValidationReport::ok();
    let token = text.lines().next().unwrap_or("").trim();
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        r.push(ValidationIssue::error("jwt", "SMART backend JWT should have header.payload.signature", "jwt-shape"));
        return r;
    }
    if parts.iter().any(|p| p.is_empty()) {
        r.push(ValidationIssue::error("jwt", "JWT segments must be non-empty", "jwt-empty-part"));
    }
    if !parts[0].starts_with("eyJ") {
        r.push(ValidationIssue::warning("jwt.header", "header usually base64url JSON starting with eyJ", "jwt-header"));
    }
    r
}
