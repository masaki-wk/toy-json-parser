use std::fmt::{self, Display};

use crate::{CodeSpan, Literal};

/// Represents a kind of JSON value.
#[derive(Debug, PartialEq, Clone)]
pub enum ValueKind {
    Array(Vec<Box<Value>>),
    Object(Vec<((String, CodeSpan), Box<Value>)>),
    Literal(Literal),
}

impl ValueKind {
    fn fmt_array<'a, I>(f: &mut fmt::Formatter, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a Box<Value>>,
    {
        write!(f, "[")?;
        for (i, v) in iter.enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            v.fmt(f)?
        }
        write!(f, "]")
    }

    fn fmt_object<'a, I>(f: &mut fmt::Formatter, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a ((String, CodeSpan), Box<Value>)>,
    {
        write!(f, "{{")?;
        for (i, ((k, _), v)) in iter.enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "\"{k}\": ")?;
            v.fmt(f)?;
        }
        write!(f, "}}")
    }

    fn fmt_literal(f: &mut fmt::Formatter, lit: &Literal) -> fmt::Result {
        lit.fmt(f)
    }

    fn fmt_impl(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Array(vec) => ValueKind::fmt_array(f, vec.iter()),
            Self::Object(vec) => ValueKind::fmt_object(f, vec.iter()),
            Self::Literal(lit) => ValueKind::fmt_literal(f, lit),
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.fmt_impl(f)
    }
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}
