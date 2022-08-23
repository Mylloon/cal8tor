pub mod models;

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

/// Print a line for the table
///
/// `pos` defines what separator should be used and can be:
/// - 0 -> top of the table
/// - 1 -> middle of the table
/// - 2 -> bottom of the table
pub fn line_table(cell_length: usize, number_cell: usize, pos: models::Position) {
    let err_msg = "Unknown position";

    // Left side
    let ls = match pos {
        models::Position::Top => models::TabChar::Jtl.val(),
        models::Position::Middle => models::TabChar::Jl.val(),
        models::Position::Bottom => models::TabChar::Jbl.val(),
        _ => panic!("{}", err_msg),
    };

    // Middle
    let ms = match pos {
        models::Position::Top => models::TabChar::Jtb.val(),
        models::Position::Middle => models::TabChar::Jm.val(),
        models::Position::Bottom => models::TabChar::Jtt.val(),
        _ => panic!("{}", err_msg),
    };

    // Right side
    let rs = match pos {
        models::Position::Top => models::TabChar::Jtr.val(),
        models::Position::Middle => models::TabChar::Jr.val(),
        models::Position::Bottom => models::TabChar::Jbr.val(),
        _ => panic!("{}", err_msg),
    };

    let line = models::TabChar::Bh.val().to_string().repeat(cell_length);

    // Print the line
    print!("\n{}{}{}", ls, line, ms);
    for _ in 2..number_cell {
        print!("{}{}", line, ms);
    }
    println!("{}{}", line, rs);
}
