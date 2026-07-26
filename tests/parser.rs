// Tests for Parser

use anyhow::{Result, bail};

use toy_json_parser::{CodeLocation, CodeSpan, Delimiter, Lexer, Literal, Parser, ParserError, Value, ValueKind};

#[test]
fn new() -> Result<()> {
    let lexer = Lexer::new("".chars());
    let _parser = Parser::new(lexer);
    Ok(())
}

fn do_parse_legal_code(input: &str, expected: Value) -> Result<()> {
    let lexer = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);
    let value = parser.parse()?;
    assert_eq!(value, expected);
    Ok(())
}

fn do_parse_illegal_code(input: &str, expected: ParserError) -> Result<()> {
    let lexer = Lexer::new(input.chars());
    let mut parser = Parser::new(lexer);
    match parser.parse() {
        Ok(_) => {
            bail!("");
        }
        Err(e) => {
            assert_eq!(e, expected);
        }
    }
    Ok(())
}

#[test]
fn parse_number() -> Result<()> {
    let input = "123";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let kind = ValueKind::Literal(Literal::Number(input.to_string()));
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_string() -> Result<()> {
    let input = "foo";
    let code = format!("\"{input}\"");
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + code.chars().count());
    let kind = ValueKind::Literal(Literal::String(input.to_string()));
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(&code, Value::new(kind, span))
}

#[test]
fn parse_true() -> Result<()> {
    let input = "true";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let kind = ValueKind::Literal(Literal::Boolean(true));
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_false() -> Result<()> {
    let input = "false";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let kind = ValueKind::Literal(Literal::Boolean(false));
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_null() -> Result<()> {
    let input = "null";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let kind = ValueKind::Literal(Literal::Null);
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_array_empty() -> Result<()> {
    let input = "[]";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let buf = Vec::new();
    let kind = ValueKind::Array(buf);
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_array_single_item() -> Result<()> {
    let input = "[0]";
    let start = CodeLocation::new(1, 1);
    let literal_start = CodeLocation::new(start.line, start.column + 1);
    let literal_end = CodeLocation::new(literal_start.line, literal_start.column + 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let mut buf = Vec::new();
    buf.push(Box::new(Value::new(
        ValueKind::Literal(Literal::Number("0".to_string())),
        CodeSpan::new(literal_start, literal_end),
    )));
    let kind = ValueKind::Array(buf);
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_array_multiple_item() -> Result<()> {
    let input = "[0, 1]";
    let start = CodeLocation::new(1, 1);
    let literal1_start = CodeLocation::new(start.line, start.column + 1);
    let literal1_end = CodeLocation::new(literal1_start.line, literal1_start.column + 1);
    let literal2_start = CodeLocation::new(literal1_end.line, literal1_end.column + 2);
    let literal2_end = CodeLocation::new(literal2_start.line, literal2_start.column + 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let mut buf = Vec::new();
    buf.push(Box::new(Value::new(
        ValueKind::Literal(Literal::Number("0".to_string())),
        CodeSpan::new(literal1_start, literal1_end),
    )));
    buf.push(Box::new(Value::new(
        ValueKind::Literal(Literal::Number("1".to_string())),
        CodeSpan::new(literal2_start, literal2_end),
    )));
    let kind = ValueKind::Array(buf);
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_object_empty() -> Result<()> {
    let input = "{}";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let buf = Vec::new();
    let kind = ValueKind::Object(buf);
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_object_single_pair() -> Result<()> {
    let input = "{\"a\": 0}";
    let start = CodeLocation::new(1, 1);
    let name_start = CodeLocation::new(start.line, start.column + 1);
    let name_end = CodeLocation::new(name_start.line, name_start.column + 3);
    let value_start = CodeLocation::new(name_end.line, name_end.column + 2);
    let value_end = CodeLocation::new(value_start.line, value_start.column + 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let mut buf = Vec::new();
    buf.push((
        ("a".to_string(), CodeSpan::new(name_start, name_end)),
        Box::new(Value::new(
            ValueKind::Literal(Literal::Number("0".to_string())),
            CodeSpan::new(value_start, value_end),
        )),
    ));
    let kind = ValueKind::Object(buf);
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_object_multiple_pair() -> Result<()> {
    let input = "{\"a\": 0, \"b\": 1}";
    let start = CodeLocation::new(1, 1);
    let name1_start = CodeLocation::new(start.line, start.column + 1);
    let name1_end = CodeLocation::new(name1_start.line, name1_start.column + 3);
    let value1_start = CodeLocation::new(name1_end.line, name1_end.column + 2);
    let value1_end = CodeLocation::new(value1_start.line, value1_start.column + 1);
    let name2_start = CodeLocation::new(value1_end.line, value1_end.column + 2);
    let name2_end = CodeLocation::new(name2_start.line, name2_start.column + 3);
    let value2_start = CodeLocation::new(name2_end.line, name2_end.column + 2);
    let value2_end = CodeLocation::new(value2_start.line, value2_start.column + 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    let mut buf = Vec::new();
    buf.push((
        ("a".to_string(), CodeSpan::new(name1_start, name1_end)),
        Box::new(Value::new(
            ValueKind::Literal(Literal::Number("0".to_string())),
            CodeSpan::new(value1_start, value1_end),
        )),
    ));
    buf.push((
        ("b".to_string(), CodeSpan::new(name2_start, name2_end)),
        Box::new(Value::new(
            ValueKind::Literal(Literal::Number("1".to_string())),
            CodeSpan::new(value2_start, value2_end),
        )),
    ));
    let kind = ValueKind::Object(buf);
    let span = CodeSpan::new(start, end);
    do_parse_legal_code(input, Value::new(kind, span))
}

#[test]
fn parse_illegal_no_token() -> Result<()> {
    let input = "";
    do_parse_illegal_code(input, ParserError::NoToken)
}

#[test]
fn parse_illegal_invalid_token() -> Result<()> {
    let input = "_";
    let pos = CodeLocation::new(1, 1);
    do_parse_illegal_code(input, ParserError::InvalidToken(input.to_string(), pos))
}

#[test]
fn parse_illegal_delimiter_in_wrong_place() -> Result<()> {
    let input = ",";
    let pos = CodeLocation::new(1, 1);
    do_parse_illegal_code(input, ParserError::DelimiterInWrongPlace(Delimiter::Comma, pos))
}

#[test]
fn parse_illegal_extra_token_at_the_end() -> Result<()> {
    let body = "0 ";
    let extra = "1";
    let input = &format!("{body}{extra}");
    let pos = CodeLocation::new(1, 1 + body.chars().count());
    do_parse_illegal_code(input, ParserError::ExtraTokenAtTheEnd(pos))
}

#[test]
fn parse_illegal_unfinished_array_no_value() -> Result<()> {
    let input = "[";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    do_parse_illegal_code(input, ParserError::UnfinishedArray(start, end))
}

#[test]
fn parse_illegal_unfinished_array_lacks_next_comma() -> Result<()> {
    let input = "[0";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    do_parse_illegal_code(input, ParserError::UnfinishedArray(start, end))
}

#[test]
fn parse_illegal_unfinished_array_lacks_next_value() -> Result<()> {
    let input = "[0,";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    do_parse_illegal_code(input, ParserError::UnfinishedArray(start, end))
}

#[test]
fn parse_illegal_unfinished_object_no_name() -> Result<()> {
    let input = "{";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    do_parse_illegal_code(input, ParserError::UnfinishedObject(start, end))
}

#[test]
fn parse_illegal_unfinished_object_lacks_next_colon() -> Result<()> {
    let input = "{\"foo\"";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    do_parse_illegal_code(input, ParserError::ObjectMemberLacksSeparator(start, end))
}

#[test]
fn parse_illegal_unfinished_object_lacks_next_value() -> Result<()> {
    let input = "{\"foo\":";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    do_parse_illegal_code(input, ParserError::ObjectMemberLacksValue(start, end))
}

#[test]
fn parse_illegal_unfinished_object_lacks_next_comma() -> Result<()> {
    let input = "{\"foo\": 0";
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + input.chars().count());
    do_parse_illegal_code(input, ParserError::UnfinishedObject(start, end))
}

#[test]
fn parse_illegal_object_name_is_not_string() -> Result<()> {
    let pre = "{";
    let name = "0";
    let post = ": 0}";
    let input = &format!("{pre}{name}{post}");
    let start = CodeLocation::new(1, 1);
    let end = CodeLocation::new(start.line, start.column + pre.chars().count());
    do_parse_illegal_code(input, ParserError::NameOfObjectMemberIsNotString(Literal::Number(name.to_string()), end))
}
