use crate::{CodeSpan, Literal};

/// Represents a kind of JSON value.
#[derive(Debug, PartialEq, Clone)]
pub enum ValueKind {
    Array(Vec<Box<Value>>),
    Object(Vec<((String, CodeSpan), Box<Value>)>),
    Literal(Literal),
}

/// Represents a JSON value.
#[derive(Debug, PartialEq, Clone)]
pub struct Value {
    pub kind: ValueKind,
    pub span: CodeSpan,
}

impl Value {
    /// Creates a new Value.
    pub const fn new(kind: ValueKind, span: CodeSpan) -> Self {
        Self { kind, span }
    }
}
