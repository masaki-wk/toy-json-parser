use std::fmt;

use crate::CodeLocation;

/// Represents a code span.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CodeSpan {
    pub start: CodeLocation,
    pub end: CodeLocation,
}

impl CodeSpan {
    /// Creates a new [`CodeSpan`].
    pub const fn new(start: CodeLocation, end: CodeLocation) -> Self {
        Self { start, end }
    }
}

impl fmt::Display for CodeSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let start = self.start;
        let end = self.end;
        write!(f, "{start}..{end}")
    }
}
