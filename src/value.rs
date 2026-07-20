use std::ops::Range;

use crate::{CodePos, Literal};

/// Represents a JSON value.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Array((Vec<Box<Value>>, Range<CodePos>)),
    Object((Vec<((std::string::String, CodePos), Box<Value>)>, Range<CodePos>)),
    Literal(Literal, CodePos),
}
