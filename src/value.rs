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
    /// Displays [`ValueKind`] via returning the helper struct `ValueDisplay`.
    pub const fn display(&self, indent_width: usize) -> ValueDisplay<'_> {
        ValueDisplay::new(ValueDisplayMode::PrettyPrint(indent_width), self)
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = ValueDisplay::new(ValueDisplayMode::ToString, self);
        disp.fmt(f)
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

    /// Displays [`Value`] via returning the helper struct `ValueDisplay`.
    pub const fn display(&self, indent_width: usize) -> ValueDisplay<'_> {
        ValueDisplay::new(ValueDisplayMode::PrettyPrint(indent_width), &self.kind)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = ValueDisplay::new(ValueDisplayMode::ToString, &self.kind);
        disp.fmt(f)
    }
}

// Helper enum for `ValueDisplay`
#[derive(Debug, Clone)]
enum ValueDisplayMode {
    ToString,
    PrettyPrint(usize),
}

/// Helper struct for printing [`Value`].
pub struct ValueDisplay<'a> {
    mode: ValueDisplayMode,
    kind: &'a ValueKind,
}

impl ValueDisplayMode {
    // Displays a header of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_header(&self, f: &mut fmt::Formatter, ch: char, is_empty: bool) -> fmt::Result {
        write!(f, "{ch}")?;
        match self {
            Self::ToString => {}
            Self::PrettyPrint(_) => {
                if !is_empty {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }

    // Displays indent.
    fn disp_indent(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        match self {
            Self::ToString => {}
            Self::PrettyPrint(indent_width) => {
                let pad_width = indent_width * depth;
                let pad = " ".repeat(pad_width);
                write!(f, "{pad}")?;
            }
        }
        Ok(())
    }

    // Displays suffix of an item of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_item_suffix(&self, f: &mut fmt::Formatter, is_last_item: bool) -> fmt::Result {
        match self {
            Self::ToString => {
                if !is_last_item {
                    write!(f, ", ")?;
                }
            }
            Self::PrettyPrint(_) => {
                if !is_last_item {
                    writeln!(f, ",")?;
                } else {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }

    // Displays a footer of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_footer(&self, f: &mut fmt::Formatter, ch: char, depth: usize, is_empty: bool) -> fmt::Result {
        if !is_empty {
            self.disp_indent(f, depth)?;
        }
        write!(f, "{ch}")
    }

    // Displays `ValueKind::Array`.
    fn disp_array<'a, I>(&self, f: &mut fmt::Formatter, depth: usize, len: usize, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a Box<Value>>,
    {
        self.disp_header(f, '[', len == 0)?;
        for (i, v) in iter.enumerate() {
            self.disp_indent(f, depth + 1)?;
            self.disp(f, depth + 1, &v.kind)?;
            self.disp_item_suffix(f, i + 1 == len)?;
        }
        self.disp_footer(f, ']', depth, len == 0)
    }

    // Displays `ValueKind::Object`.
    fn disp_object<'a, I>(&self, f: &mut fmt::Formatter, depth: usize, len: usize, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a ((String, CodeSpan), Box<Value>)>,
    {
        self.disp_header(f, '{', len == 0)?;
        for (i, ((k, _), v)) in iter.enumerate() {
            self.disp_indent(f, depth + 1)?;
            write!(f, r#""{k}": "#)?;
            self.disp(f, depth + 1, &v.kind)?;
            self.disp_item_suffix(f, i + 1 == len)?;
        }
        self.disp_footer(f, '}', depth, len == 0)
    }

    // Displays `ValueKind::Literal`.
    fn disp_literal(&self, f: &mut fmt::Formatter, lit: &Literal) -> fmt::Result {
        lit.fmt(f)
    }

    // Displays `ValueKind`.
    fn disp(&self, f: &mut fmt::Formatter, depth: usize, kind: &ValueKind) -> fmt::Result {
        match kind {
            ValueKind::Array(vec) => self.disp_array(f, depth, vec.len(), vec.iter()),
            ValueKind::Object(vec) => self.disp_object(f, depth, vec.len(), vec.iter()),
            ValueKind::Literal(lit) => self.disp_literal(f, lit),
        }
    }
}

impl<'a> ValueDisplay<'a> {
    // Creates a new `ValueDisplay`.
    const fn new(mode: ValueDisplayMode, kind: &'a ValueKind) -> Self {
        Self { mode, kind }
    }
}

impl<'a> fmt::Display for ValueDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.mode.disp(f, 0, self.kind)
    }
}
