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

/// Represents a diagnostic by a parser.
#[derive(Debug, PartialEq, Clone)]
pub struct ParserDiag {}

impl<T> Parser<T>
where
    T: Iterator<Item = Token>,
{
    /// Creates a new parser.
    pub fn new(tokens: T) -> Self {
        Self { tokens: tokens.peekable() }
    }

    /// Parses a code.
    pub fn parse(&mut self) -> Result<Value, ParserDiag> {
        let value = self.parse_value()?;
        match self.tokens.next() {
            Some(_) => Err(ParserDiag {}),
            None => Ok(value),
        }
    }

    // Parses a value.
    fn parse_value(&mut self) -> Result<Value, ParserDiag> {
        enum TokenCategory {
            BeginArray,
            BeginObject,
            Literal(Literal),
        }
        let (token_category, pos) = if let Some(token) = self.tokens.peek() {
            match token.kind.clone() {
                TokenKind::Delimiter(Delimiter::LeftBracket) => Ok((TokenCategory::BeginArray, token.pos)),
                TokenKind::Delimiter(Delimiter::LeftBrace) => Ok((TokenCategory::BeginObject, token.pos)),
                TokenKind::Literal(l) => Ok((TokenCategory::Literal(l), token.pos)),
                TokenKind::Invalid(_) => Err(ParserDiag {}),
                _ => Err(ParserDiag {}),
            }
        } else {
            Err(ParserDiag {})
        }?;
        self.tokens.next();
        match token_category {
            TokenCategory::BeginArray => self.parse_rest_of_array(pos),
            TokenCategory::BeginObject => self.parse_rest_of_object(pos),
            TokenCategory::Literal(l) => Ok(Value::Literal(l)),
        }
    }

    // Parses a rest of the array.
    fn parse_rest_of_array(&mut self, start: CodePos) -> Result<Value, ParserDiag> {
        let mut buf: Vec<Box<Value>> = Vec::new();
        let end = loop {
            match self.tokens.peek() {
                Some(token) if token.kind == TokenKind::Delimiter(Delimiter::RightBracket) => {
                    let mut pos = token.pos;
                    pos.advance_column();
                    self.tokens.next();
                    break pos;
                }
                Some(token) if token.kind == TokenKind::Delimiter(Delimiter::Comma) => {
                    if buf.is_empty() {
                        Err(ParserDiag {})
                    } else {
                        self.tokens.next();
                        Ok(())
                    }
                }
                Some(_) => Ok(()),
                None => Err(ParserDiag {}),
            }?;
            let item = self.parse_value()?;
            buf.push(Box::new(item));
        };
        Ok(Value::Array((buf, Range { start, end })))
    }

    // Parses a rest of the object.
    fn parse_rest_of_object(&mut self, start: CodePos) -> Result<Value, ParserDiag> {
        let mut buf: Vec<(String, Box<Value>)> = Vec::new();
        let end = loop {
            match self.tokens.peek() {
                Some(token) if token.kind == TokenKind::Delimiter(Delimiter::RightBrace) => {
                    let mut pos = token.pos;
                    pos.advance_column();
                    self.tokens.next();
                    break pos;
                }
                Some(token) if token.kind == TokenKind::Delimiter(Delimiter::Comma) => {
                    if buf.is_empty() {
                        Err(ParserDiag {})
                    } else {
                        self.tokens.next();
                        Ok(())
                    }
                }
                Some(_) => Ok(()),
                None => Err(ParserDiag {}),
            }?;
            let pair = self.parse_pair_for_object()?;
            buf.push((pair.0, Box::new(pair.1)));
        };
        Ok(Value::Object((buf, Range { start, end })))
    }

    // Parses a pair of the object.
    fn parse_pair_for_object(&mut self) -> Result<(String, Value), ParserDiag> {
        let name = match self.tokens.peek() {
            Some(token) => match token.kind.clone() {
                TokenKind::Literal(Literal::String(s)) => {
                    self.tokens.next();
                    Ok(s)
                }
                _ => Err(ParserDiag {}),
            },
            _ => Err(ParserDiag {}),
        }?;
        match self.tokens.peek() {
            Some(token) if token.kind == TokenKind::Delimiter(Delimiter::Colon) => {
                self.tokens.next();
                Ok(())
            }
            _ => Err(ParserDiag {}),
        }?;
        let value = self.parse_value()?;
        Ok((name, value))
    }
}
