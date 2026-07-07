use std::ops::Range;

use crate::CodePos;

/// Represents a kind of token.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// `[`
    LeftBracket,

    /// `]`
    RightBracket,

    /// `{`
    LeftBrace,

    /// `}`
    RightBrace,

    /// `:`
    Colon,

    /// `,`
    Comma,

    /// Number
    Number(std::string::String),

    /// String
    String(std::string::String),

    /// `true` or `false`
    Boolean(bool),

    /// `null`
    Null,

    /// Invalid character
    Invalid(std::string::String),
}

/// Represents a token.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range<CodePos>,
}
