use crate::CodePos;

/// Represents a code span.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CodeSpan {
    pub start: CodePos,
    pub end: CodePos,
}

impl CodeSpan {
    /// Creates a new `CodeSpan`.
    pub const fn new(start: CodePos, end: CodePos) -> Self {
        Self { start, end }
    }
}
