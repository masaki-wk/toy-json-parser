// Tests for CodeSpan

use anyhow::Result;

use toy_json_parser::{CodeLocation, CodeSpan};

#[test]
fn new() -> Result<()> {
    let loc = CodeLocation::new(1, 1);
    let _span = CodeSpan::new(loc, loc);
    Ok(())
}

#[test]
fn display() -> Result<()> {
    let start_line = 1;
    let start_column = 2;
    let end_line = 1;
    let end_column = 3;
    let start = CodeLocation::new(start_line, start_column);
    let end = CodeLocation::new(end_line, end_column);
    let span = CodeSpan::new(start, end);
    let expected = format!("[Ln {start_line}, Col {start_column}]..[Ln {end_line}, Col {end_column}]");
    assert_eq!(span.to_string(), expected);
    Ok(())
}
