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
    /// A JSON number literal, e.g. `0`, `123`, or `-1.2`.
    ///
    /// [`Lexer`] stores the source text of the number without converting it to a numeric type.
    ///
    /// [`Lexer`]: crate::Lexer
    ///
    Number(std::string::String),

    /// A JSON string literal, e.g. `foo` or `bar`.
    ///
    /// The surrounding quotes are removed, but escape sequences are preserved.
    String(std::string::String),

    /// A JSON boolean literal: `true` or `false`.
    Boolean(bool),

    /// The JSON `null` literal.
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(s) => write!(f, "{s}"),
            Self::String(s) => write!(f, r#""{s}""#),
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
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Delimiter(delim) => delim.fmt(f),
            Self::Literal(lit) => lit.fmt(f),
        }
    }
}

/// Represents a JSON token.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// The kind of the token.
    pub kind: TokenKind,

    /// The span of the token in the JSON source text.
    pub span: CodeSpan,
}

impl Token {
    /// Creates a new [`Token`].
    pub const fn new(kind: TokenKind, span: CodeSpan) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CodeLocation;

    fn do_display_token_test(kind: TokenKind, expected: &str) {
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + expected.chars().count());
        let span = CodeSpan::new(start, end);
        let target = Token::new(kind, span);
        assert_eq!(&target.to_string(), expected);
    }

    #[test]
    fn display_token_delimiter_left_bracket() {
        let expected = "[";
        let kind = TokenKind::Delimiter(Delimiter::LeftBracket);
        do_display_token_test(kind, expected)
    }

    #[test]
    fn display_token_delimiter_right_bracket() {
        let expected = "]";
        let kind = TokenKind::Delimiter(Delimiter::RightBracket);
        do_display_token_test(kind, expected)
    }

    #[test]
    fn display_token_delimiter_left_brace() {
        let expected = "{";
        let kind = TokenKind::Delimiter(Delimiter::LeftBrace);
        do_display_token_test(kind, expected)
    }

    #[test]
    fn display_token_delimiter_right_brace() {
        let expected = "}";
        let kind = TokenKind::Delimiter(Delimiter::RightBrace);
        do_display_token_test(kind, expected)
    }

    #[test]
    fn display_token_delimiter_colon() {
        let expected = ":";
        let kind = TokenKind::Delimiter(Delimiter::Colon);
        do_display_token_test(kind, expected)
    }

    #[test]
    fn display_token_delimiter_comma() {
        let expected = ",";
        let kind = TokenKind::Delimiter(Delimiter::Comma);
        do_display_token_test(kind, expected)
    }

    #[test]
    fn display_token_literal_number() {
        let s = "123";
        let kind = TokenKind::Literal(Literal::Number(s.to_string()));
        do_display_token_test(kind, s)
    }

    #[test]
    fn display_token_literal_string() {
        let s = "foo";
        let expected = format!(r#""{s}""#);
        let kind = TokenKind::Literal(Literal::String(s.to_string()));
        do_display_token_test(kind, &expected)
    }

    #[test]
    fn display_token_literal_boolean_false() {
        let expected = "false";
        let kind = TokenKind::Literal(Literal::Boolean(false));
        do_display_token_test(kind, expected)
    }

    #[test]
    fn display_token_literal_boolean_true() {
        let expected = "true";
        let kind = TokenKind::Literal(Literal::Boolean(true));
        do_display_token_test(kind, expected)
    }

    #[test]
    fn display_token_literal_null() {
        let expected = "null";
        let kind = TokenKind::Literal(Literal::Null);
        do_display_token_test(kind, expected)
    }
}
