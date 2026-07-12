use std::iter::Peekable;
use std::ops::Range;

use crate::{CodePos, Token, TokenKind, Value, ValueKind};

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
            Some(_) => Err(ParserDiag {}),
            None => Ok(value),
        }
    }

    // Parses a value.
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
            TokenCategory::BeginArray => self.parse_rest_of_array(range.start),
            TokenCategory::BeginObject => self.parse_rest_of_object(range.start),
        }
    }

    // Parses a rest of the array.
    fn parse_rest_of_array(&mut self, start: CodePos) -> Result<Value, ParserDiag> {
        let mut buf: Vec<Box<ValueKind>> = Vec::new();
        let end = loop {
            match self.tokens.peek() {
                Some(token) if token.kind == TokenKind::RightBracket => {
                    let end = token.range.end;
                    self.tokens.next();
                    break end;
                }
                Some(token) if token.kind == TokenKind::Comma => {
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
            buf.push(Box::new(item.kind));
        };
        Ok(Value {
            kind: ValueKind::Array(buf),
            range: Range { start, end },
        })
    }

    // Parses a rest of the object.
    fn parse_rest_of_object(&mut self, start: CodePos) -> Result<Value, ParserDiag> {
        let mut buf: Vec<(String, Box<ValueKind>)> = Vec::new();
        let end = loop {
            match self.tokens.peek() {
                Some(token) if token.kind == TokenKind::RightBrace => {
                    let end = token.range.end;
                    self.tokens.next();
                    break end;
                }
                Some(token) if token.kind == TokenKind::Comma => {
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
        Ok(Value {
            kind: ValueKind::Object(buf),
            range: Range { start, end },
        })
    }

    // Parses a pair of the object.
    fn parse_pair_for_object(&mut self) -> Result<(String, ValueKind), ParserDiag> {
        let name = match self.tokens.peek() {
            Some(token) => match token.kind.clone() {
                TokenKind::String(s) => {
                    self.tokens.next();
                    Ok(s)
                }
                _ => Err(ParserDiag {}),
            },
            _ => Err(ParserDiag {}),
        }?;
        match self.tokens.peek() {
            Some(token) if token.kind == TokenKind::Colon => {
                self.tokens.next();
                Ok(())
            }
            _ => Err(ParserDiag {}),
        }?;
        let value = self.parse_value()?;
        Ok((name, value.kind))
    }
}
