use crate::{CodePos, Delimiter, Literal, Token};

/// Represents a parser error.
#[derive(Debug, PartialEq, Clone)]
pub enum ParserError {
    NoToken,
    InvalidToken(Token),
    DelimiterInWrongPlace(Delimiter, CodePos),
    UnfinishedArray(CodePos, CodePos),
    UnfinishedObject(CodePos, CodePos),
    NameOfObjectMemberIsNotString(Literal, CodePos),
    ObjectMemberLacksSeparator(CodePos, CodePos),
    ObjectMemberLacksValue(CodePos, CodePos),
    ExtraTokenAtTheEnd(CodePos),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParserError {}
