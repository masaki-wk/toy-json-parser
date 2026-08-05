// Tests for Value

use anyhow::Result;

use toy_json_parser::{CodeLocation, CodeSpan, Literal, Value, ValueKind};

fn do_display_value_test(kind: ValueKind, expected: &str) -> Result<()> {
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + expected.chars().count());
    let span = CodeSpan::new(start, end);
    let target = Value::new(kind, span);
    assert_eq!(&target.to_string(), expected);
    Ok(())
}

#[test]
fn display_value_literal_null() -> Result<()> {
    let expected = "null";
    let kind = ValueKind::Literal(Literal::Null);
    do_display_value_test(kind, expected)
}

#[test]
fn display_value_array_empty() -> Result<()> {
    let expected = "[]";
    let kind = ValueKind::Array(vec![]);
    do_display_value_test(kind, expected)
}

#[test]
fn display_value_array_single_item() -> Result<()> {
    let expected = "[null]";
    let loc = CodeLocation::new(1, 1);
    let span = CodeSpan::new(loc, loc);
    let item = Value::new(ValueKind::Literal(Literal::Null), span);
    let kind = ValueKind::Array(vec![Box::new(item)]);
    do_display_value_test(kind, expected)
}

#[test]
fn display_value_array_multiple_item() -> Result<()> {
    let expected = "[null, null]";
    let loc = CodeLocation::new(1, 1);
    let span = CodeSpan::new(loc, loc);
    let item = Value::new(ValueKind::Literal(Literal::Null), span);
    let kind = ValueKind::Array(vec![Box::new(item.clone()), Box::new(item)]);
    do_display_value_test(kind, expected)
}

#[test]
fn display_value_object_empty() -> Result<()> {
    let expected = "{}";
    let kind = ValueKind::Object(vec![]);
    do_display_value_test(kind, expected)
}

#[test]
fn display_value_object_single_item() -> Result<()> {
    let expected = "{\"a\": null}";
    let name = "a".to_string();
    let loc = CodeLocation::new(1, 1);
    let span = CodeSpan::new(loc, loc);
    let item = Value::new(ValueKind::Literal(Literal::Null), span);
    let kind = ValueKind::Object(vec![((name, span), Box::new(item))]);
    do_display_value_test(kind, expected)
}

#[test]
fn display_value_object_multiple_item() -> Result<()> {
    let expected = "{\"a\": null, \"b\": null}";
    let name0 = "a".to_string();
    let name1 = "b".to_string();
    let loc = CodeLocation::new(1, 1);
    let span = CodeSpan::new(loc, loc);
    let item = Value::new(ValueKind::Literal(Literal::Null), span);
    let kind = ValueKind::Object(vec![((name0, span), Box::new(item.clone())), ((name1, span), Box::new(item))]);
    do_display_value_test(kind, expected)
}
