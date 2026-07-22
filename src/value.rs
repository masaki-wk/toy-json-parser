use std::ops::Range;

use crate::{CodePos, Literal};

/// Represents a kind of JSON value.
#[derive(Debug, PartialEq, Clone)]
pub enum ValueKind {
    Array(Vec<Box<Value>>),
    Object(Vec<((std::string::String, Range<CodePos>), Box<Value>)>),
    Literal(Literal),
}

/// Represents a JSON value.
#[derive(Debug, PartialEq, Clone)]
pub struct Value {
    pub kind: ValueKind,
    pub range: Range<CodePos>,
}
