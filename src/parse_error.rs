use std::fmt;

use crate::{CodeLocation, Delimiter, LexicalErrorKind, Literal};

/// Represents a parse error by [`Parser`].
///
/// [`Parser`]: crate::Parser
///
#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    /// Lexical error occurred at [`CodeLocation`].
    LexicalError(LexicalErrorKind, String, CodeLocation),

    /// The input was empty.
    EmptyInput,

    /// Unexpected trailing token at [`CodeLocation`] after the end of the JSON input.
    TrailingToken(CodeLocation),

    /// Unexpected delimiter encountered at [`CodeLocation`].
    UnexpectedDelimiter(Delimiter, CodeLocation),

    /// Array was missing its closing `]`.
    #[allow(missing_docs)]
    UnclosedArray { array_start: CodeLocation, error_at: CodeLocation },

    /// Object was missing its closing `}`.
    #[allow(missing_docs)]
    UnclosedObject { object_start: CodeLocation, error_at: CodeLocation },

    /// Array was missing `,` separator between array elements.
    #[allow(missing_docs)]
    ArrayMissingSeparator { array_start: CodeLocation, error_at: CodeLocation },

    /// Object was missing `,` separator between object members.
    #[allow(missing_docs)]
    ObjectMissingSeparator { object_start: CodeLocation, error_at: CodeLocation },

    /// Object member name was not a string.
    /// The offending literal and its location are reported in the first and second fields.
    ObjectMemberNameNotString(Literal, CodeLocation),

    /// Object member was missing `:` separator after the member name.
    #[allow(missing_docs)]
    ObjectMemberMissingSeparator { member_start: CodeLocation, error_at: CodeLocation },

    /// Object member was missing value after `:` separator.
    #[allow(missing_docs)]
    ObjectMemberMissingValue { member_start: CodeLocation, error_at: CodeLocation },

    /// Maximum nesting depth exceeded at [`CodeLocation`].
    ///
    /// The maximum nesting depth of the [`Parser`] instance is set by [`new()`] or [`with_max_depth()`].
    /// The definition of nesting depth in [`Parser`] is:
    ///
    /// - The root value is depth 0.
    /// - Each nested array/object increments the depth.
    ///
    /// [`Parser`]: crate::Parser
    /// [`new()`]: crate::Parser::new
    /// [`with_max_depth()`]: crate::Parser::with_max_depth
    ///
    NestingDepthExceeded(CodeLocation),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ParseError {}
