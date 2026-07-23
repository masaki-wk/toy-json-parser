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
        let (value, _) = self.parse_value()?;
        match self.lexer.next() {
            Some(token) => Err(ParserError::ExtraTokenAtTheEnd(token.range.start)),
            None => Ok(value),
        }
    }

    // Parses a value and then returns the value and the range of the last token.
    fn parse_value(&mut self) -> Result<(Value, Range<CodePos>), ParserError> {
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
            TokenCategory::BeginArray => self.parse_rest_of_array(token_range),
            TokenCategory::BeginObject => self.parse_rest_of_object(token_range),
            TokenCategory::Literal(lit) => {
                let kind = ValueKind::Literal(lit);
                Ok((
                    Value {
                        kind,
                        range: token_range.clone(),
                    },
                    token_range,
                ))
            }
        }
    }

    // Parses a rest of the array.
    fn parse_rest_of_array(&mut self, begin_array_token_range: Range<CodePos>) -> Result<(Value, Range<CodePos>), ParserError> {
        let mut buf: Vec<Box<Value>> = Vec::new();
        let mut last_token_range = begin_array_token_range.clone();
        let (end, last_token_range) = loop {
            let token_pos = last_token_range.end;
            let token = self
                .lexer
                .peek()
                .ok_or(ParserError::UnfinishedArray(begin_array_token_range.start, token_pos))?;
            let token_range = token.range.clone();
            match token.kind {
                TokenKind::Delimiter(Delimiter::RightBracket) => {
                    let end = token_range.end;
                    self.lexer.next();
                    break (end, token_range);
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
            let (item, last_token_range_new) = self.parse_value().map_err(|e| match e {
                ParserError::NoToken => ParserError::UnfinishedArray(begin_array_token_range.start, token_range.end),
                _ => e,
            })?;
            buf.push(Box::new(item));
            last_token_range = last_token_range_new;
        };
        Ok((
            Value {
                kind: ValueKind::Array(buf),
                range: begin_array_token_range.start..end,
            },
            last_token_range,
        ))
    }

    // Parses a rest of the object.
    fn parse_rest_of_object(&mut self, begin_object_token_range: Range<CodePos>) -> Result<(Value, Range<CodePos>), ParserError> {
        let mut buf: Vec<((String, Range<CodePos>), Box<Value>)> = Vec::new();
        let mut last_token_range = begin_object_token_range.clone();
        let (end, last_token_range) = loop {
            let token_pos = last_token_range.end;
            let token = self
                .lexer
                .peek()
                .ok_or(ParserError::UnfinishedObject(begin_object_token_range.start, token_pos))?;
            let token_range = token.range.clone();
            match token.kind {
                TokenKind::Delimiter(Delimiter::RightBrace) => {
                    let end = token_range.end;
                    self.lexer.next();
                    break (end, token_range);
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
            let (name_pair, value, last_token_range_new) = self.parse_pair_for_object(begin_object_token_range.clone(), token_range)?;
            buf.push((name_pair, Box::new(value)));
            last_token_range = last_token_range_new;
        };
        Ok((
            Value {
                kind: ValueKind::Object(buf),
                range: begin_object_token_range.start..end,
            },
            last_token_range,
        ))
    }

    // Parses a pair of the object.
    fn parse_pair_for_object(
        &mut self,
        begin_object_token_range: Range<CodePos>,
        last_token_range: Range<CodePos>,
    ) -> Result<((String, Range<CodePos>), Value, Range<CodePos>), ParserError> {
        let token_for_name = self
            .lexer
            .next()
            .ok_or(ParserError::UnfinishedObject(begin_object_token_range.start, last_token_range.end))?;
        let name = match token_for_name.kind {
            TokenKind::Literal(Literal::String(s)) => Ok(s),
            TokenKind::Literal(lit) => Err(ParserError::NameOfObjectMemberIsNotString(lit, token_for_name.range.start)),
            TokenKind::Delimiter(delim) => Err(ParserError::DelimiterInWrongPlace(delim, token_for_name.range.start)),
            TokenKind::Invalid(s) => Err(ParserError::InvalidToken(s, token_for_name.range.start)),
        }?;
        let token_for_colon = self.lexer.next().ok_or(ParserError::ObjectMemberLacksSeparator(
            begin_object_token_range.start,
            token_for_name.range.end,
        ))?;
        match token_for_colon.kind {
            TokenKind::Delimiter(Delimiter::Colon) => Ok(()),
            TokenKind::Invalid(s) => Err(ParserError::InvalidToken(s, token_for_colon.range.start)),
            _ => Err(ParserError::ObjectMemberLacksSeparator(
                begin_object_token_range.start,
                token_for_colon.range.start,
            )),
        }?;
        let (value, last_token_range) = self.parse_value().map_err(|e| match e {
            ParserError::NoToken => ParserError::ObjectMemberLacksValue(begin_object_token_range.start, token_for_colon.range.end),
            _ => e,
        })?;
        Ok(((name, token_for_name.range), value, last_token_range))
    }
}
