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
