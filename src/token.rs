use std::fmt;

use crate::CodeSpan;

/// Represents a JSON delimiter.
#[derive(Debug, PartialEq, Clone)]
pub enum Delimiter {
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
}

impl fmt::Display for Delimiter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            Self::LeftBracket => '[',
            Self::RightBracket => ']',
            Self::LeftBrace => '{',
            Self::RightBrace => '}',
            Self::Colon => ':',
            Self::Comma => ',',
        };
        write!(f, "{ch}")
    }
}

/// Represents a JSON literal.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    /// A number, e.g. `0`, `123`
    Number(std::string::String),

    /// A string, e.g. `"foo"`, `"bar"`
    String(std::string::String),

    /// `true` or `false`
    Boolean(bool),

    /// `null`
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(s) => write!(f, "{s}"),
            Self::String(s) => write!(f, "\"{s}\""),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Null => write!(f, "null"),
        }
    }
}

/// Represents a kind of JSON token.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// Delimiter
    Delimiter(Delimiter),

    /// Literal
    Literal(Literal),

    /// Invalid
    Invalid(String),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Delimiter(delim) => delim.fmt(f),
            Self::Literal(lit) => lit.fmt(f),
            Self::Invalid(s) => write!(f, "{}", s),
        }
    }
}

/// Represents a JSON token.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: CodeSpan,
}

impl Token {
    /// Creates a new Token.
    pub const fn new(kind: TokenKind, span: CodeSpan) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}
