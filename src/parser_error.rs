use crate::{CodePos, Delimiter, Token};

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
