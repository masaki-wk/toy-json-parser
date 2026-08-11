// Tests for CodeLocation

use anyhow::Result;

use toy_json_parser::CodeLocation;

#[test]
fn new() -> Result<()> {
    let _loc = CodeLocation::new(1, 1);
    Ok(())
}

#[test]
fn display() -> Result<()> {
    let line = 1;
    let column = 2;
    let loc = CodeLocation::new(line, column);
    let expected = format!("[Ln {line}, Col {column}]");
    assert_eq!(loc.to_string(), expected);
    Ok(())
}
