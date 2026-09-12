use std::fmt;

use crate::CodeLocation;

/// Represents a kind of lexical error by [`Lexer`].
///
/// [`Lexer`]: crate::Lexer
///
#[derive(Debug, PartialEq, Clone)]
pub enum LexicalErrorKind {
    /// Unexpected character was found.
    UnexpectedChar,

    /// Unquoted string was found.
    UnquotedString,

    /// String was not terminated by a closing `"`.
    UnterminatedString,

    /// Number has a leading zero before integer component digits.
    NumberContainsLeadingZero,

    /// Number was missing integer digits.
    NumberMissingIntegerDigits,

    /// Number has a decimal point but no fraction digits.
    NumberMissingFractionDigits,

    /// Number has an exponent indicator but no exponent digits.
    NumberMissingExponentDigits,

    /// String contains an unescaped control character.
    StringContainsUnescapedControlChar,

    /// String contains an invalid escape sequence.
    StringContainsInvalidEscapeSequence,

    /// String contains a `\u` escape that is not followed by four hexadecimal digits.
    StringContainsInvalidUnicodeEscape,
}

/// Represents a lexical error by [`Lexer`].
///
/// [`Lexer`]: crate::Lexer
///
#[derive(Debug, PartialEq, Clone)]
pub struct LexicalError {
    /// The kind of the error.
    pub kind: LexicalErrorKind,

    /// The substring of the JSON source text related to the error, stored as-is.
    pub string: String,

    /// The starting location of the error.
    pub location: CodeLocation,
}

impl LexicalError {
    /// Creates a new [`LexicalError`].
    pub const fn new(kind: LexicalErrorKind, string: String, location: CodeLocation) -> Self {
        Self { kind, string, location }
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for LexicalError {}
