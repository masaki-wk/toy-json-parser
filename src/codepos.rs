/// Represents a code position.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CodePos {
    pub line: usize,
    pub column: usize,
}

impl CodePos {
    /// Creates a new `CodePos`.
    pub fn new(line: usize, column: usize) -> Self {
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
