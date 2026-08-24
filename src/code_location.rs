use std::fmt;

/// Represents a code location.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CodeLocation {
    /// The line number (1-indexed).
    pub line: usize,

    /// The column number (1-indexed).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _loc = CodeLocation::new(1, 1);
    }

    #[test]
    fn display() {
        let line = 1;
        let column = 2;
        let loc = CodeLocation::new(line, column);
        let expected = format!("[Ln {line}, Col {column}]");
        assert_eq!(loc.to_string(), expected);
    }
}
