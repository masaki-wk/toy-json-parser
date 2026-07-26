use crate::{CodeLocation, Delimiter, Literal};

/// Represents a parser error.
#[derive(Debug, PartialEq, Clone)]
pub enum ParserError {
    NoToken,
    InvalidToken(String, CodeLocation),
    DelimiterInWrongPlace(Delimiter, CodeLocation),
    UnfinishedArray(CodeLocation, CodeLocation),
    UnfinishedObject(CodeLocation, CodeLocation),
    NameOfObjectMemberIsNotString(Literal, CodeLocation),
    ObjectMemberLacksSeparator(CodeLocation, CodeLocation),
    ObjectMemberLacksValue(CodeLocation, CodeLocation),
    ExtraTokenAtTheEnd(CodeLocation),
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParserError {}
