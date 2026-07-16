use crate::CodePos;

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

/// Represents a JSON literal.
#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    /// Number
    Number(std::string::String),

    /// String
    String(std::string::String),

    /// `true` or `false`
    Boolean(bool),

    /// `null`
    Null,
}

impl Literal {
    /// Returns the number of characters in the literal.
    pub fn len(&self) -> usize {
        match self {
            Self::Number(s) => s.chars().count(),
            Self::String(s) => s.chars().count() + 2,
            Self::Boolean(b) => {
                if *b {
                    4
                } else {
                    5
                }
            }
            Self::Null => 4,
        }
    }

    /// Returns `true` if the literal has a length of 0.
    pub const fn is_empty(&self) -> bool {
        false
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

impl TokenKind {
    /// Returns the number of characters in the token.
    pub fn len(&self) -> usize {
        match self {
            Self::Delimiter(_) => 1,
            Self::Literal(l) => l.len(),
            Self::Invalid(s) => s.chars().count(),
        }
    }

    /// Returns `true` if the literal has a length of 0.
    pub const fn is_empty(&self) -> bool {
        false
    }
}

/// Represents a JSON token.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: CodePos,
}
