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
    NameOfObjectMemberIsNotString(Literal, CodeLocation),
    ObjectMemberLacksSeparator(CodeLocation, CodeLocation),
    ObjectMemberLacksValue(CodeLocation, CodeLocation),
    ExtraTokenAtTheEnd(CodeLocation),
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
}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    /// Creates a new [`Parser`].
    pub fn new(lexer: T) -> Self {
        Self { lexer: lexer.peekable() }
    }

    /// Parses a code.
    pub fn parse(&mut self) -> Result<Value, ParseError> {
        let (value, _) = self.parse_value()?;
        match self.lexer.next() {
            Some(token) => Err(ParseError::ExtraTokenAtTheEnd(token.span.start)),
            None => Ok(value),
        }
    }

    // Parses a value and then returns the value and the span of the last token.
    fn parse_value(&mut self) -> Result<(Value, CodeSpan), ParseError> {
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
        match token_category {
            TokenCategory::BeginArray => self.parse_rest_of_array(token_span),
            TokenCategory::BeginObject => self.parse_rest_of_object(token_span),
            TokenCategory::Literal(lit) => {
                let kind = ValueKind::Literal(lit);
                Ok((Value::new(kind, token_span), token_span))
            }
        }
    }

    // Parses a rest of the array.
    fn parse_rest_of_array(&mut self, begin_array_token_span: CodeSpan) -> Result<(Value, CodeSpan), ParseError> {
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
            let (item, last_token_span_new) = self.parse_value().map_err(|e| match e {
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
    fn parse_rest_of_object(&mut self, begin_object_token_span: CodeSpan) -> Result<(Value, CodeSpan), ParseError> {
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
                _ => Ok(()),
            }?;
            let (name_pair, value, last_token_span_new) = self.parse_pair_for_object(begin_object_token_span, token_span)?;
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
        let (value, last_token_span) = self.parse_value().map_err(|e| match e {
            ParseError::NoToken => ParseError::ObjectMemberLacksValue(begin_object_token_span.start, token_for_colon.span.end),
            _ => e,
        })?;
        Ok(((name, token_for_name.span), value, last_token_span))
    }
}
