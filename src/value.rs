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
    // Displays `ValueKind::Array`.
    fn disp_array<'a, I>(f: &mut fmt::Formatter, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a Box<Value>>,
    {
        write!(f, "[")?;
        for (i, v) in iter.enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            v.kind.disp(f)?;
        }
        write!(f, "]")
    }

    // Displays `ValueKind::Object`.
    fn disp_object<'a, I>(f: &mut fmt::Formatter, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a ((String, CodeSpan), Box<Value>)>,
    {
        write!(f, "{{")?;
        for (i, ((k, _), v)) in iter.enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, r#""{k}": "#)?;
            v.kind.disp(f)?;
        }
        write!(f, "}}")
    }

    // Displays `ValueKind::Literal`.
    fn disp_literal(f: &mut fmt::Formatter, lit: &Literal) -> fmt::Result {
        lit.fmt(f)
    }

    // Displays `ValueKind`.
    fn disp(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Array(vec) => Self::disp_array(f, vec.iter()),
            Self::Object(vec) => Self::disp_object(f, vec.iter()),
            Self::Literal(lit) => Self::disp_literal(f, lit),
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.disp(f)
    }
}

/// Represents a JSON value.
#[derive(Debug, PartialEq, Clone)]
pub struct Value {
    pub kind: ValueKind,
    pub span: CodeSpan,
}

impl Value {
    /// Creates a new [`Value`].
    pub const fn new(kind: ValueKind, span: CodeSpan) -> Self {
        Self { kind, span }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}
