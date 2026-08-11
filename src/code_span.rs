use std::fmt;

/// Represents a code location.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CodeLocation {
    pub line: usize,
    pub column: usize,
}

impl CodeLocation {
    /// Creates a new [`CodeLocation`].
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl fmt::Display for CodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = self.line;
        let column = self.column;
        write!(f, "[Ln {line}, Col {column}]")
    }
}

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
