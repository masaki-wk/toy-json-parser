use std::ops::Range;

use crate::CodePos;

/// Represents a kind of token.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// `[`
    LeftSquareBracket,

    /// `]`
    RightSquareBracket,

    /// `{`
    LeftCurlyBracket,

    /// `}`
    RightCurlyBracket,

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
}

/// Represents a token.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Range<CodePos>,
}
