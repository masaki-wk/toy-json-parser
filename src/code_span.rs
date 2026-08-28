use std::fmt;

use crate::CodeLocation;

/// Represents a code span.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CodeSpan {
    /// The start location of the span (inclusive).
    pub start: CodeLocation,

    /// The end location of the span (exclusive).
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
        let start_line = self.start.line;
        let end_line = self.end.line;
        let start_column = self.start.column;
        let end_column = self.end.column;
        if start_line == end_line {
            write!(f, "[Ln {start_line}, Col {start_column}-{end_column}]")
        } else {
            write!(f, "[Ln {start_line}, Col {start_column} - Ln {end_line}, Col {end_column}]")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let loc = CodeLocation::new(1, 1);
        let _span = CodeSpan::new(loc, loc);
    }

    #[test]
    fn display_same_line() {
        let start_line = 1;
        let start_column = 2;
        let end_line = 1;
        let end_column = 3;
        let start = CodeLocation::new(start_line, start_column);
        let end = CodeLocation::new(end_line, end_column);
        let span = CodeSpan::new(start, end);
        let expected = format!("[Ln {start_line}, Col {start_column}-{end_column}]");
        assert_eq!(span.to_string(), expected);
    }

    #[test]
    fn display_lines() {
        let start_line = 1;
        let start_column = 2;
        let end_line = 2;
        let end_column = 1;
        let start = CodeLocation::new(start_line, start_column);
        let end = CodeLocation::new(end_line, end_column);
        let span = CodeSpan::new(start, end);
        let expected = format!("[Ln {start_line}, Col {start_column} - Ln {end_line}, Col {end_column}]");
        assert_eq!(span.to_string(), expected);
    }
}
