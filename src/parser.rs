use std::ops::Range;

use crate::{CodePos, Delimiter, Lexer, Literal, ParserError, TokenKind, Value, ValueKind};

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
    T: Iterator<Item = char>,
{
    lexer: Lexer<T>,
}

impl<T> Parser<T>
where
    T: Iterator<Item = char>,
{
    /// Creates a new parser.
    pub fn new(lexer: Lexer<T>) -> Self {
        Self { lexer }
    }

    /// Parses a code.
    pub fn parse(&mut self) -> Result<Value, ParserError> {
        let value = self.parse_value()?;
        match self.lexer.next() {
            Some(token) => Err(ParserError::ExtraTokenAtTheEnd(token.range.start)),
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
        let (token_category, token_range) = if let Some(token) = self.lexer.next() {
            match token.kind {
                TokenKind::Delimiter(Delimiter::LeftBracket) => Ok((TokenCategory::BeginArray, token.range)),
                TokenKind::Delimiter(Delimiter::LeftBrace) => Ok((TokenCategory::BeginObject, token.range)),
                TokenKind::Delimiter(delim) => Err(ParserError::DelimiterInWrongPlace(delim, token.range.start)),
                TokenKind::Literal(lit) => Ok((TokenCategory::Literal(lit), token.range)),
                TokenKind::Invalid(s) => Err(ParserError::InvalidToken(s, token.range.start)),
            }
        } else {
            Err(ParserError::NoToken)
        }?;
        match token_category {
            TokenCategory::BeginArray => self.parse_rest_of_array(token_range.start),
            TokenCategory::BeginObject => self.parse_rest_of_object(token_range.start),
            TokenCategory::Literal(lit) => {
                let kind = ValueKind::Literal(lit);
                Ok(Value { kind, range: token_range })
            }
        }
    }

    // Parses a rest of the array.
    fn parse_rest_of_array(&mut self, start: CodePos) -> Result<Value, ParserError> {
        let mut buf: Vec<Box<Value>> = Vec::new();
        let end = loop {
            let token_pos = self.lexer.position();
            let token = self.lexer.peek().ok_or(ParserError::UnfinishedArray(start, token_pos))?;
            match token.kind {
                TokenKind::Delimiter(Delimiter::RightBracket) => {
                    let end = token.range.end;
                    self.lexer.next();
                    break end;
                }
                TokenKind::Delimiter(Delimiter::Comma) => {
                    if buf.is_empty() {
                        Err(ParserError::DelimiterInWrongPlace(Delimiter::Comma, token.range.start))
                    } else {
                        self.lexer.next();
                        Ok(())
                    }
                }
                _ => Ok(()),
            }?;
            let item = self.parse_value().map_err(|e| match e {
                ParserError::NoToken => ParserError::UnfinishedArray(start, self.lexer.position()),
                _ => e,
            })?;
            buf.push(Box::new(item));
        };
        Ok(Value {
            kind: ValueKind::Array(buf),
            range: start..end,
        })
    }

    // Parses a rest of the object.
    fn parse_rest_of_object(&mut self, start: CodePos) -> Result<Value, ParserError> {
        let mut buf: Vec<((String, Range<CodePos>), Box<Value>)> = Vec::new();
        let end = loop {
            let token_pos = self.lexer.position();
            let token = self.lexer.peek().ok_or(ParserError::UnfinishedObject(start, token_pos))?;
            match token.kind {
                TokenKind::Delimiter(Delimiter::RightBrace) => {
                    let end = token.range.end;
                    self.lexer.next();
                    break end;
                }
                TokenKind::Delimiter(Delimiter::Comma) => {
                    if buf.is_empty() {
                        Err(ParserError::DelimiterInWrongPlace(Delimiter::Comma, token.range.start))
                    } else {
                        self.lexer.next();
                        Ok(())
                    }
                }
                _ => Ok(()),
            }?;
            let pair = self.parse_pair_for_object(start)?;
            buf.push((pair.0, Box::new(pair.1)));
        };
        Ok(Value {
            kind: ValueKind::Object(buf),
            range: start..end,
        })
    }

    // Parses a pair of the object.
    fn parse_pair_for_object(&mut self, start: CodePos) -> Result<((String, Range<CodePos>), Value), ParserError> {
        let token_for_name = self.lexer.next().ok_or(ParserError::UnfinishedObject(start, self.lexer.position()))?;
        let name = match token_for_name.kind {
            TokenKind::Literal(Literal::String(s)) => Ok(s),
            TokenKind::Literal(lit) => Err(ParserError::NameOfObjectMemberIsNotString(lit, token_for_name.range.start)),
            TokenKind::Delimiter(delim) => Err(ParserError::DelimiterInWrongPlace(delim, token_for_name.range.start)),
            TokenKind::Invalid(s) => Err(ParserError::InvalidToken(s, token_for_name.range.start)),
        }?;
        let token_for_colon = self.lexer.next().ok_or(ParserError::ObjectMemberLacksSeparator(start, self.lexer.position()))?;
        match token_for_colon.kind {
            TokenKind::Delimiter(Delimiter::Colon) => Ok(()),
            TokenKind::Invalid(s) => Err(ParserError::InvalidToken(s, token_for_colon.range.start)),
            _ => Err(ParserError::ObjectMemberLacksSeparator(start, token_for_colon.range.start)),
        }?;
        let value = self.parse_value().map_err(|e| match e {
            ParserError::NoToken => ParserError::ObjectMemberLacksValue(start, self.lexer.position()),
            _ => e,
        })?;
        Ok(((name, token_for_name.range), value))
    }
}
