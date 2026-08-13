// Tests for CodeLocation

use toy_json_parser::CodeLocation;

#[test]
fn new() {
    let _loc = CodeLocation::new(1, 1);
}

#[test]
fn display() {
    let line = 1;
    let column = 2;
    let loc = CodeLocation::new(line, column);
    let expected = format!("[Ln {line}, Col {column}]");
    assert_eq!(loc.to_string(), expected);
}
