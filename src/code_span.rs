/// Represents a code location.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CodeLocation {
    pub line: usize,
    pub column: usize,
}

impl CodeLocation {
    /// Creates a new `CodeLocation`.
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    /// Advances a line.
    pub fn advance_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }

    /// Advances a column.
    pub fn advance_column(&mut self) {
        self.column += 1;
    }
}

/// Represents a code span.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CodeSpan {
    pub start: CodeLocation,
    pub end: CodeLocation,
}

impl CodeSpan {
    /// Creates a new `CodeSpan`.
    pub const fn new(start: CodeLocation, end: CodeLocation) -> Self {
        Self { start, end }
    }
}
