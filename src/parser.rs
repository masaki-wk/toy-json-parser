use std::fmt;
use std::iter::Peekable;

use crate::{CodeLocation, CodeSpan, Delimiter, Literal, Token, TokenKind, Value, ValueKind};

/// Represents a parse error by [`Parser`].
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    NoToken,
    InvalidToken(String, CodeLocation),
    DelimiterInWrongPlace(Delimiter, CodeLocation),
    UnfinishedArray(CodeLocation, CodeLocation),
    UnfinishedObject(CodeLocation, CodeLocation),
    ArrayLacksSeparator(CodeLocation, CodeLocation),
    ObjectLacksSeparator(CodeLocation, CodeLocation),
    NameOfObjectMemberIsNotString(Literal, CodeLocation),
    ObjectMemberLacksSeparator(CodeLocation, CodeLocation),
    ObjectMemberLacksValue(CodeLocation, CodeLocation),
    ExtraTokenAtTheEnd(CodeLocation),
    NestingDepthExceeded(CodeLocation),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ParseError {}

/// Represents a JSON parser.
///
/// # Examples
///
/// ```
/// # use toy_json_parser::{Lexer, Parser};
/// # fn test() -> Option<()> {
/// let lexer = Lexer::new("[]".chars());
/// let mut parser = Parser::new(lexer);
/// assert!(parser.parse().is_ok());
/// # Some(())
/// # }
/// ```
///
#[derive(Debug, Clone)]
pub struct Parser<T>
where
    T: Iterator<Item = Token>,
{
    lexer: Peekable<T>,
    max_depth: usize,
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    /// Creates a new [`Parser`].
    pub fn new(lexer: T) -> Self {
        Self {
            lexer: lexer.peekable(),
            max_depth: 512,
        }
    }

    /// Parses a code.
    pub fn parse(&mut self) -> Result<Value, ParseError> {
        let (value, _) = self.parse_value(0)?;
        match self.lexer.next() {
            Some(token) => Err(ParseError::ExtraTokenAtTheEnd(token.span.start)),
            None => Ok(value),
        }
    }

    // Parses a value and then returns the value and the span of the last token.
    fn parse_value(&mut self, current_depth: usize) -> Result<(Value, CodeSpan), ParseError> {
        enum TokenCategory {
            BeginArray,
            BeginObject,
            Literal(Literal),
        }
        let (token_category, token_span) = if let Some(token) = self.lexer.next() {
            match token.kind {
                TokenKind::Delimiter(Delimiter::LeftBracket) => Ok((TokenCategory::BeginArray, token.span)),
                TokenKind::Delimiter(Delimiter::LeftBrace) => Ok((TokenCategory::BeginObject, token.span)),
                TokenKind::Delimiter(delim) => Err(ParseError::DelimiterInWrongPlace(delim, token.span.start)),
                TokenKind::Literal(lit) => Ok((TokenCategory::Literal(lit), token.span)),
                TokenKind::Invalid(s) => Err(ParseError::InvalidToken(s, token.span.start)),
            }
        } else {
            Err(ParseError::NoToken)
        }?;
        if current_depth > self.max_depth {
            return Err(ParseError::NestingDepthExceeded(token_span.start));
        }
        match token_category {
            TokenCategory::BeginArray => self.parse_rest_of_array(current_depth, token_span),
            TokenCategory::BeginObject => self.parse_rest_of_object(current_depth, token_span),
            TokenCategory::Literal(lit) => {
                let kind = ValueKind::Literal(lit);
                Ok((Value::new(kind, token_span), token_span))
            }
        }
    }

    // Parses a rest of the array.
    fn parse_rest_of_array(&mut self, current_depth: usize, begin_array_token_span: CodeSpan) -> Result<(Value, CodeSpan), ParseError> {
        let mut buf: Vec<Box<Value>> = Vec::new();
        let mut last_token_span = begin_array_token_span;
        let (end, last_token_span) = loop {
            let token_loc = last_token_span.end;
            let token = self.lexer.peek().ok_or(ParseError::UnfinishedArray(begin_array_token_span.start, token_loc))?;
            let token_span = token.span;
            match token.kind {
                TokenKind::Delimiter(Delimiter::RightBracket) => {
                    let end = token_span.end;
                    self.lexer.next();
                    break (end, token_span);
                }
                TokenKind::Delimiter(Delimiter::Comma) => {
                    if buf.is_empty() {
                        Err(ParseError::DelimiterInWrongPlace(Delimiter::Comma, token.span.start))
                    } else {
                        self.lexer.next();
                        Ok(())
                    }
                }
                _ => {
                    if buf.is_empty() {
                        Ok(())
                    } else {
                        Err(ParseError::ArrayLacksSeparator(begin_array_token_span.start, token_loc))
                    }
                }
            }?;
            let (item, last_token_span_new) = self.parse_value(current_depth + 1).map_err(|e| match e {
                ParseError::NoToken => ParseError::UnfinishedArray(begin_array_token_span.start, token_span.end),
                _ => e,
            })?;
            buf.push(Box::new(item));
            last_token_span = last_token_span_new;
        };
        Ok((
            Value::new(ValueKind::Array(buf), CodeSpan::new(begin_array_token_span.start, end)),
            last_token_span,
        ))
    }

    // Parses a rest of the object.
    fn parse_rest_of_object(&mut self, current_depth: usize, begin_object_token_span: CodeSpan) -> Result<(Value, CodeSpan), ParseError> {
        let mut buf: Vec<((String, CodeSpan), Box<Value>)> = Vec::new();
        let mut last_token_span = begin_object_token_span;
        let (end, last_token_span) = loop {
            let token_loc = last_token_span.end;
            let token = self
                .lexer
                .peek()
                .ok_or(ParseError::UnfinishedObject(begin_object_token_span.start, token_loc))?;
            let token_span = token.span;
            match token.kind {
                TokenKind::Delimiter(Delimiter::RightBrace) => {
                    let end = token_span.end;
                    self.lexer.next();
                    break (end, token_span);
                }
                TokenKind::Delimiter(Delimiter::Comma) => {
                    if buf.is_empty() {
                        Err(ParseError::DelimiterInWrongPlace(Delimiter::Comma, token.span.start))
                    } else {
                        self.lexer.next();
                        Ok(())
                    }
                }
                _ => {
                    if buf.is_empty() {
                        Ok(())
                    } else {
                        Err(ParseError::ObjectLacksSeparator(begin_object_token_span.start, token_loc))
                    }
                }
            }?;
            let (name_pair, value, last_token_span_new) = self.parse_pair_for_object(current_depth, begin_object_token_span, token_span)?;
            buf.push((name_pair, Box::new(value)));
            last_token_span = last_token_span_new;
        };
        Ok((
            Value::new(ValueKind::Object(buf), CodeSpan::new(begin_object_token_span.start, end)),
            last_token_span,
        ))
    }

    // Parses a pair of the object.
    fn parse_pair_for_object(
        &mut self,
        current_depth: usize,
        begin_object_token_span: CodeSpan,
        last_token_span: CodeSpan,
    ) -> Result<((String, CodeSpan), Value, CodeSpan), ParseError> {
        let token_for_name = self
            .lexer
            .next()
            .ok_or(ParseError::UnfinishedObject(begin_object_token_span.start, last_token_span.end))?;
        let name = match token_for_name.kind {
            TokenKind::Literal(Literal::String(s)) => Ok(s),
            TokenKind::Literal(lit) => Err(ParseError::NameOfObjectMemberIsNotString(lit, token_for_name.span.start)),
            TokenKind::Delimiter(delim) => Err(ParseError::DelimiterInWrongPlace(delim, token_for_name.span.start)),
            TokenKind::Invalid(s) => Err(ParseError::InvalidToken(s, token_for_name.span.start)),
        }?;
        let token_for_colon = self
            .lexer
            .next()
            .ok_or(ParseError::ObjectMemberLacksSeparator(begin_object_token_span.start, token_for_name.span.end))?;
        match token_for_colon.kind {
            TokenKind::Delimiter(Delimiter::Colon) => Ok(()),
            TokenKind::Invalid(s) => Err(ParseError::InvalidToken(s, token_for_colon.span.start)),
            _ => Err(ParseError::ObjectMemberLacksSeparator(
                begin_object_token_span.start,
                token_for_colon.span.start,
            )),
        }?;
        let (value, last_token_span) = self.parse_value(current_depth + 1).map_err(|e| match e {
            ParseError::NoToken => ParseError::ObjectMemberLacksValue(begin_object_token_span.start, token_for_colon.span.end),
            _ => e,
        })?;
        Ok(((name, token_for_name.span), value, last_token_span))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Lexer;

    fn do_parse_legal_code(input: &str, expected: Value) {
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        let value = parser.parse().unwrap();
        assert_eq!(value, expected);
    }

    fn do_parse_illegal_code(input: &str, expected: ParseError) {
        let lexer = Lexer::new(input.chars());
        let mut parser = Parser::new(lexer);
        match parser.parse() {
            Ok(_) => {
                panic!();
            }
            Err(e) => match e {
                ParseError::NestingDepthExceeded(_) => {}
                _ => {
                    assert_eq!(e, expected);
                }
            },
        }
    }

    #[test]
    fn new() {
        let lexer = Lexer::new("".chars());
        let _parser = Parser::new(lexer);
    }

    #[test]
    fn parse_number() {
        let input = "123";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        let kind = ValueKind::Literal(Literal::Number(input.to_string()));
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_string() {
        let input = "foo";
        let code = format!(r#""{input}""#);
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + code.chars().count());
        let kind = ValueKind::Literal(Literal::String(input.to_string()));
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(&code, Value::new(kind, span))
    }

    #[test]
    fn parse_true() {
        let input = "true";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        let kind = ValueKind::Literal(Literal::Boolean(true));
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_false() {
        let input = "false";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        let kind = ValueKind::Literal(Literal::Boolean(false));
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_null() {
        let input = "null";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        let kind = ValueKind::Literal(Literal::Null);
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_array_empty() {
        let input = "[]";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        let buf = Vec::new();
        let kind = ValueKind::Array(buf);
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_array_single_item() {
        let input = "[0]";
        let start = CodeLocation::new(1, 1);
        let literal_start = CodeLocation::new(start.line, start.column + 1);
        let literal_end = CodeLocation::new(literal_start.line, literal_start.column + 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        let buf = vec![Box::new(Value::new(
            ValueKind::Literal(Literal::Number("0".to_string())),
            CodeSpan::new(literal_start, literal_end),
        ))];
        let kind = ValueKind::Array(buf);
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_array_multiple_item() {
        let input = "[0, 1]";
        let start = CodeLocation::new(1, 1);
        let literal1_start = CodeLocation::new(start.line, start.column + 1);
        let literal1_end = CodeLocation::new(literal1_start.line, literal1_start.column + 1);
        let literal2_start = CodeLocation::new(literal1_end.line, literal1_end.column + 2);
        let literal2_end = CodeLocation::new(literal2_start.line, literal2_start.column + 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        let buf = vec![
            Box::new(Value::new(
                ValueKind::Literal(Literal::Number("0".to_string())),
                CodeSpan::new(literal1_start, literal1_end),
            )),
            Box::new(Value::new(
                ValueKind::Literal(Literal::Number("1".to_string())),
                CodeSpan::new(literal2_start, literal2_end),
            )),
        ];
        let kind = ValueKind::Array(buf);
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_object_empty() {
        let input = "{}";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        let buf = Vec::new();
        let kind = ValueKind::Object(buf);
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_object_single_pair() {
        let input = r#"{"a": 0}"#;
        let start = CodeLocation::new(1, 1);
        let name_start = CodeLocation::new(start.line, start.column + 1);
        let name_end = CodeLocation::new(name_start.line, name_start.column + 3);
        let value_start = CodeLocation::new(name_end.line, name_end.column + 2);
        let value_end = CodeLocation::new(value_start.line, value_start.column + 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        let buf = vec![(
            ("a".to_string(), CodeSpan::new(name_start, name_end)),
            Box::new(Value::new(
                ValueKind::Literal(Literal::Number("0".to_string())),
                CodeSpan::new(value_start, value_end),
            )),
        )];
        let kind = ValueKind::Object(buf);
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_object_multiple_pair() {
        let input = r#"{"a": 0, "b": 1}"#;
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
        let buf = vec![
            (
                ("a".to_string(), CodeSpan::new(name1_start, name1_end)),
                Box::new(Value::new(
                    ValueKind::Literal(Literal::Number("0".to_string())),
                    CodeSpan::new(value1_start, value1_end),
                )),
            ),
            (
                ("b".to_string(), CodeSpan::new(name2_start, name2_end)),
                Box::new(Value::new(
                    ValueKind::Literal(Literal::Number("1".to_string())),
                    CodeSpan::new(value2_start, value2_end),
                )),
            ),
        ];
        let kind = ValueKind::Object(buf);
        let span = CodeSpan::new(start, end);
        do_parse_legal_code(input, Value::new(kind, span))
    }

    #[test]
    fn parse_illegal_no_token() {
        let input = "";
        do_parse_illegal_code(input, ParseError::NoToken)
    }

    #[test]
    fn parse_illegal_invalid_token() {
        let input = "_";
        let loc = CodeLocation::new(1, 1);
        do_parse_illegal_code(input, ParseError::InvalidToken(input.to_string(), loc))
    }

    #[test]
    fn parse_illegal_delimiter_in_wrong_place() {
        let input = ",";
        let loc = CodeLocation::new(1, 1);
        do_parse_illegal_code(input, ParseError::DelimiterInWrongPlace(Delimiter::Comma, loc))
    }

    #[test]
    fn parse_illegal_extra_token_at_the_end() {
        let body = "0 ";
        let extra = "1";
        let input = &format!("{body}{extra}");
        let loc = CodeLocation::new(1, 1 + body.chars().count());
        do_parse_illegal_code(input, ParseError::ExtraTokenAtTheEnd(loc))
    }

    #[test]
    fn parse_illegal_unfinished_array_no_value() {
        let input = "[";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_parse_illegal_code(input, ParseError::UnfinishedArray(start, end))
    }

    #[test]
    fn parse_illegal_unfinished_array_lacks_next_comma() {
        let input = "[0";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_parse_illegal_code(input, ParseError::UnfinishedArray(start, end))
    }

    #[test]
    fn parse_illegal_unfinished_array_lacks_next_value() {
        let input = "[0,";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_parse_illegal_code(input, ParseError::UnfinishedArray(start, end))
    }

    #[test]
    fn parse_illegal_array_lacks_separator() {
        let pre = "[0";
        let post = " 1]";
        let input = &format!("{pre}{post}");
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + pre.chars().count());
        do_parse_illegal_code(input, ParseError::ArrayLacksSeparator(start, end))
    }

    #[test]
    fn parse_illegal_unfinished_object_no_name() {
        let input = "{";
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_parse_illegal_code(input, ParseError::UnfinishedObject(start, end))
    }

    #[test]
    fn parse_illegal_unfinished_object_lacks_next_colon() {
        let input = r#"{"foo""#;
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_parse_illegal_code(input, ParseError::ObjectMemberLacksSeparator(start, end))
    }

    #[test]
    fn parse_illegal_unfinished_object_lacks_next_value() {
        let input = r#"{"foo":"#;
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_parse_illegal_code(input, ParseError::ObjectMemberLacksValue(start, end))
    }

    #[test]
    fn parse_illegal_unfinished_object_lacks_next_comma() {
        let input = r#"{"foo": 0"#;
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + input.chars().count());
        do_parse_illegal_code(input, ParseError::UnfinishedObject(start, end))
    }

    #[test]
    fn parse_illegal_object_lacks_separator() {
        let pre = r#"{"foo": 0"#;
        let post = r#" "bar": 1}"#;
        let input = &format!("{pre}{post}");
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + pre.chars().count());
        do_parse_illegal_code(input, ParseError::ObjectLacksSeparator(start, end))
    }

    #[test]
    fn parse_illegal_object_name_is_not_string() {
        let pre = "{";
        let name = "0";
        let post = ": 0}";
        let input = &format!("{pre}{name}{post}");
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + pre.chars().count());
        do_parse_illegal_code(input, ParseError::NameOfObjectMemberIsNotString(Literal::Number(name.to_string()), end))
    }

    #[test]
    fn parse_illegal_nesting_depth_exceeded() {
        let nesting_depth = 1024;
        let input = &format!("{}{}", "[".repeat(nesting_depth), "]".repeat(nesting_depth));
        let dummy = CodeLocation::new(1, 1);
        do_parse_illegal_code(input, ParseError::NestingDepthExceeded(dummy))
    }
}
