use std::ops::Range;

use crate::CodePos;

/// Represents a kind of JSON value.
#[derive(Debug, PartialEq, Clone)]
pub enum ValueKind {
    Array(Vec<Box<ValueKind>>),
    Object(Vec<(String, Box<ValueKind>)>),
    Number(std::string::String),
    String(std::string::String),
    Boolean(bool),
    Null,
}

/// Represents a JSON value.
#[derive(Debug, PartialEq, Clone)]
pub struct Value {
    pub kind: ValueKind,
    pub range: Range<CodePos>,
}
