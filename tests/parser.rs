// Tests for Parser

use anyhow::{Result, bail};

use toy_json_parser::{CodePos, Delimiter, Lexer, Literal, Parser, ParserError, Value};

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
    let s = "123";
    let pos = CodePos { line: 1, column: 1 };
    do_parse_legal_code(s, Value::Literal(Literal::Number(s.to_string()), pos))
}

#[test]
fn parse_string() -> Result<()> {
    let s = "foo";
    let code = format!("\"{s}\"");
    let pos = CodePos { line: 1, column: 1 };
    do_parse_legal_code(&code, Value::Literal(Literal::String(s.to_string()), pos))
}

#[test]
fn parse_true() -> Result<()> {
    let pos = CodePos { line: 1, column: 1 };
    do_parse_legal_code("true", Value::Literal(Literal::Boolean(true), pos))
}

#[test]
fn parse_false() -> Result<()> {
    let pos = CodePos { line: 1, column: 1 };
    do_parse_legal_code("false", Value::Literal(Literal::Boolean(false), pos))
}

#[test]
fn parse_null() -> Result<()> {
    let pos = CodePos { line: 1, column: 1 };
    do_parse_legal_code("null", Value::Literal(Literal::Null, pos))
}

#[test]
fn parse_array_empty() -> Result<()> {
    let input = "[]";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: 1,
        column: start.column + input.chars().count(),
    };
    let buf = Vec::new();
    do_parse_legal_code(input, Value::Array((buf, start..end)))
}

#[test]
fn parse_array_single_item() -> Result<()> {
    let input = "[0]";
    let start = CodePos { line: 1, column: 1 };
    let literal_pos = CodePos {
        line: start.line,
        column: start.column + 1,
    };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    let mut buf = Vec::new();
    buf.push(Box::new(Value::Literal(Literal::Number("0".to_string()), literal_pos)));
    do_parse_legal_code(input, Value::Array((buf, start..end)))
}

#[test]
fn parse_array_multiple_item() -> Result<()> {
    let input = "[0, 1]";
    let start = CodePos { line: 1, column: 1 };
    let literal1_pos = CodePos {
        line: start.line,
        column: start.column + 1,
    };
    let literal2_pos = CodePos {
        line: start.line,
        column: literal1_pos.column + 3,
    };
    let end = CodePos {
        line: 1,
        column: start.column + input.chars().count(),
    };
    let mut buf = Vec::new();
    buf.push(Box::new(Value::Literal(Literal::Number("0".to_string()), literal1_pos)));
    buf.push(Box::new(Value::Literal(Literal::Number("1".to_string()), literal2_pos)));
    do_parse_legal_code(input, Value::Array((buf, start..end)))
}

#[test]
fn parse_object_empty() -> Result<()> {
    let input = "{}";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    let buf = Vec::new();
    do_parse_legal_code(input, Value::Object((buf, start..end)))
}

#[test]
fn parse_object_single_pair() -> Result<()> {
    let input = "{\"a\": 0}";
    let start = CodePos { line: 1, column: 1 };
    let value_pos = CodePos {
        line: start.line,
        column: start.column + 6,
    };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    let mut buf = Vec::new();
    buf.push(("a".to_string(), Box::new(Value::Literal(Literal::Number("0".to_string()), value_pos))));
    do_parse_legal_code(input, Value::Object((buf, start..end)))
}

#[test]
fn parse_object_multiple_pair() -> Result<()> {
    let input = "{\"a\": 0, \"b\": 1}";
    let start = CodePos { line: 1, column: 1 };
    let value1_pos = CodePos {
        line: start.line,
        column: start.column + 6,
    };
    let value2_pos = CodePos {
        line: start.line,
        column: value1_pos.column + 8,
    };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    let mut buf = Vec::new();
    buf.push(("a".to_string(), Box::new(Value::Literal(Literal::Number("0".to_string()), value1_pos))));
    buf.push(("b".to_string(), Box::new(Value::Literal(Literal::Number("1".to_string()), value2_pos))));
    do_parse_legal_code(input, Value::Object((buf, start..end)))
}

#[test]
fn parse_illegal_no_token() -> Result<()> {
    let input = "";
    do_parse_illegal_code(input, ParserError::NoToken)
}

#[test]
fn parse_illegal_invalid_token() -> Result<()> {
    let input = "_";
    let pos = CodePos { line: 1, column: 1 };
    do_parse_illegal_code(input, ParserError::InvalidToken(input.to_string(), pos))
}

#[test]
fn parse_illegal_delimiter_in_wrong_place() -> Result<()> {
    let input = ",";
    let pos = CodePos { line: 1, column: 1 };
    do_parse_illegal_code(input, ParserError::DelimiterInWrongPlace(Delimiter::Comma, pos))
}

#[test]
fn parse_illegal_extra_token_at_the_end() -> Result<()> {
    let body = "0 ";
    let extra = "1";
    let input = &format!("{body}{extra}");
    let pos = CodePos {
        line: 1,
        column: 1 + body.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::ExtraTokenAtTheEnd(pos))
}

#[test]
fn parse_illegal_unfinished_array_no_value() -> Result<()> {
    let input = "[";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::UnfinishedArray(start, end))
}

#[test]
fn parse_illegal_unfinished_array_lacks_next_comma() -> Result<()> {
    let input = "[0";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::UnfinishedArray(start, end))
}

#[test]
fn parse_illegal_unfinished_array_lacks_next_value() -> Result<()> {
    let input = "[0,";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::UnfinishedArray(start, end))
}

#[test]
fn parse_illegal_unfinished_object_no_name() -> Result<()> {
    let input = "{";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::UnfinishedObject(start, end))
}

#[test]
fn parse_illegal_unfinished_object_lacks_next_colon() -> Result<()> {
    let input = "{\"foo\"";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::ObjectMemberLacksSeparator(start, end))
}

#[test]
fn parse_illegal_unfinished_object_lacks_next_value() -> Result<()> {
    let input = "{\"foo\":";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::ObjectMemberLacksValue(start, end))
}

#[test]
fn parse_illegal_unfinished_object_lacks_next_comma() -> Result<()> {
    let input = "{\"foo\": 0";
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: start.line,
        column: start.column + input.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::UnfinishedObject(start, end))
}

#[test]
fn parse_illegal_object_name_is_not_string() -> Result<()> {
    let pre = "{";
    let name = "0";
    let post = ": 0}";
    let input = &format!("{pre}{name}{post}");
    let start = CodePos { line: 1, column: 1 };
    let end = CodePos {
        line: start.line,
        column: start.column + pre.chars().count(),
    };
    do_parse_illegal_code(input, ParserError::NameOfObjectMemberIsNotString(Literal::Number(name.to_string()), end))
}
