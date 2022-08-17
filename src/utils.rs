/// Panic if an error happened
pub fn check_errors(html: &String, loc: &str) {
    match html {
        t if t.contains(&err_code(429)) => panic!(
            "URL: {} • HTTP 429: Slow down - Rate limited (too many access attempts detected)",
            loc
        ),
        _ => (),
    }
}

/// Create String error code
fn err_code(code: i32) -> String {
    format!("HTTP Code : {}", code)
}

/// Return the string if exists, otherwise return empty string
pub fn display_option(o: &Option<String>) -> String {
    match o {
        Some(s) => s.to_string(),
        None => String::new(),
    }
}
