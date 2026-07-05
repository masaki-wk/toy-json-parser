/// Represents a code position.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct CodePos {
    pub line: usize,
    pub column: usize,
}
