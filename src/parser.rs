use std::iter::Peekable;

use crate::{Token, TokenKind, Value, ValueKind};

/// Represents a parser.
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
            Some(_) => todo!(),
            None => Ok(value),
        }
    }

    /// Parses a value.
    fn parse_value(&mut self) -> Result<Value, ParserDiag> {
        enum TokenCategory {
            BeginArray,
            BeginObject,
            Value(ValueKind),
        }
        let (token_category, range) = if let Some(token) = self.tokens.peek() {
            match token.kind.clone() {
                TokenKind::String(s) => Ok((TokenCategory::Value(ValueKind::String(s)), token.range.clone())),
                TokenKind::Number(s) => Ok((TokenCategory::Value(ValueKind::Number(s)), token.range.clone())),
                TokenKind::Boolean(b) => Ok((TokenCategory::Value(ValueKind::Boolean(b)), token.range.clone())),
                TokenKind::Null => Ok((TokenCategory::Value(ValueKind::Null), token.range.clone())),
                TokenKind::Invalid(_) => Err(ParserDiag {}),
                TokenKind::LeftBracket => Ok((TokenCategory::BeginArray, token.range.clone())),
                TokenKind::LeftBrace => Ok((TokenCategory::BeginObject, token.range.clone())),
                _ => Err(ParserDiag {}),
            }
        } else {
            Err(ParserDiag {})
        }?;
        self.tokens.next();
        match token_category {
            TokenCategory::Value(kind) => Ok(Value { kind, range }),
            _ => todo!(),
        }
    }
}
