use std::iter::Peekable;
use std::ops::Range;

use crate::{CodePos, Delimiter, Literal, Token, TokenKind, Value};

/// Represents a parser.
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
    tokens: Peekable<T>,
}

/// Represents a parser error.
#[derive(Debug, PartialEq, Clone)]
pub enum ParserError {
    NoToken,
    InvalidToken(Token),
    DelimiterInWrongPlace(Delimiter, CodePos),
    UnfinishedArray(CodePos, CodePos),
    UnfinishedObject(CodePos, CodePos),
    NameOfObjectMemberIsNotString(CodePos, Token),
    ObjectMemberLacksSeparator(CodePos, Token),
    ObjectMemberLacksValue(CodePos, CodePos),
    ExtraTokenAtTheEnd(Token),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParserError {}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    /// Creates a new parser.
    pub fn new(tokens: T) -> Self {
        Self { tokens: tokens.peekable() }
    }

    /// Parses a code.
    pub fn parse(&mut self) -> Result<Value, ParserError> {
        let value = self.parse_value()?;
        match self.tokens.next() {
            Some(token) => Err(ParserError::ExtraTokenAtTheEnd(token)),
            None => Ok(value),
        }
    }

    // Parses a value.
    fn parse_value(&mut self) -> Result<Value, ParserError> {
        enum TokenCategory {
            BeginArray,
            BeginObject,
            Literal(Literal),
        }
        let (token_category, pos) = if let Some(token) = self.tokens.next() {
            match token.kind {
                TokenKind::Delimiter(Delimiter::LeftBracket) => Ok((TokenCategory::BeginArray, token.pos)),
                TokenKind::Delimiter(Delimiter::LeftBrace) => Ok((TokenCategory::BeginObject, token.pos)),
                TokenKind::Delimiter(delim) => Err(ParserError::DelimiterInWrongPlace(delim, token.pos)),
                TokenKind::Literal(lit) => Ok((TokenCategory::Literal(lit), token.pos)),
                TokenKind::Invalid(_) => Err(ParserError::InvalidToken(token)),
            }
        } else {
            Err(ParserError::NoToken)
        }?;
        match token_category {
            TokenCategory::BeginArray => self.parse_rest_of_array(pos),
            TokenCategory::BeginObject => self.parse_rest_of_object(pos),
            TokenCategory::Literal(lit) => Ok(Value::Literal(lit)),
        }
    }

    // Parses a rest of the array.
    fn parse_rest_of_array(&mut self, start: CodePos) -> Result<Value, ParserError> {
        let mut buf: Vec<Box<Value>> = Vec::new();
        let end = loop {
            let token = self.tokens.peek().ok_or(ParserError::UnfinishedArray(start, CodePos { line: 1, column: 1 }))?;
            match token.kind {
                TokenKind::Delimiter(Delimiter::RightBracket) => {
                    let mut pos = token.pos;
                    pos.advance_column();
                    self.tokens.next();
                    break pos;
                }
                TokenKind::Delimiter(Delimiter::Comma) => {
                    if buf.is_empty() {
                        Err(ParserError::DelimiterInWrongPlace(Delimiter::Comma, token.pos))
                    } else {
                        self.tokens.next();
                        Ok(())
                    }
                }
                _ => Ok(()),
            }?;
            let item = self.parse_value().map_err(|e| match e {
                ParserError::NoToken => ParserError::UnfinishedArray(start, CodePos { line: 1, column: 1 }),
                _ => e,
            })?;
            buf.push(Box::new(item));
        };
        Ok(Value::Array((buf, Range { start, end })))
    }

    // Parses a rest of the object.
    fn parse_rest_of_object(&mut self, start: CodePos) -> Result<Value, ParserError> {
        let mut buf: Vec<(String, Box<Value>)> = Vec::new();
        let end = loop {
            let token = self.tokens.peek().ok_or(ParserError::UnfinishedObject(start, CodePos { line: 1, column: 1 }))?;
            match token.kind {
                TokenKind::Delimiter(Delimiter::RightBrace) => {
                    let mut pos = token.pos;
                    pos.advance_column();
                    self.tokens.next();
                    break pos;
                }
                TokenKind::Delimiter(Delimiter::Comma) => {
                    if buf.is_empty() {
                        Err(ParserError::DelimiterInWrongPlace(Delimiter::Comma, token.pos))
                    } else {
                        self.tokens.next();
                        Ok(())
                    }
                }
                _ => Ok(()),
            }?;
            let pair = self.parse_pair_for_object(start)?;
            buf.push((pair.0, Box::new(pair.1)));
        };
        Ok(Value::Object((buf, Range { start, end })))
    }

    // Parses a pair of the object.
    fn parse_pair_for_object(&mut self, start: CodePos) -> Result<(String, Value), ParserError> {
        let token_for_name = self.tokens.next().ok_or(ParserError::UnfinishedObject(start, CodePos { line: 1, column: 1 }))?;
        let token_for_name_pos = token_for_name.pos;
        let name = match token_for_name.kind {
            TokenKind::Literal(Literal::String(s)) => Ok(s),
            _ => Err(ParserError::NameOfObjectMemberIsNotString(start, token_for_name)),
        }?;
        let token_for_colon = self.tokens.next().ok_or(ParserError::UnfinishedObject(start, token_for_name_pos))?;
        match token_for_colon.kind {
            TokenKind::Delimiter(Delimiter::Colon) => Ok(()),
            _ => Err(ParserError::ObjectMemberLacksSeparator(start, token_for_colon)),
        }?;
        let value = self.parse_value().map_err(|e| match e {
            ParserError::NoToken => ParserError::ObjectMemberLacksValue(start, CodePos { line: 1, column: 1 }),
            _ => e,
        })?;
        Ok((name, value))
    }
}
