use std::fmt::{self, Display};

use crate::{CodeSpan, Literal};

/// Represents a kind of JSON value.
#[derive(Debug, PartialEq, Clone)]
pub enum ValueKind {
    Array(Vec<Box<Value>>),
    Object(Vec<((String, CodeSpan), Box<Value>)>),
    Literal(Literal),
}

// Helper enum for `ValueKind::disp()`
#[derive(Debug, Clone)]
enum ValueKindDisplayMode {
    ToString,
    PrettyPrint(usize),
}

impl ValueKind {
    // Displays a header of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_header(f: &mut fmt::Formatter, mode: &ValueKindDisplayMode, ch: char, is_empty: bool) -> fmt::Result {
        write!(f, "{ch}")?;
        match mode {
            ValueKindDisplayMode::ToString => {}
            ValueKindDisplayMode::PrettyPrint(_) => {
                if !is_empty {
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }

    // Displays indent.
    fn disp_indent(f: &mut fmt::Formatter, mode: &ValueKindDisplayMode, depth: usize) -> fmt::Result {
        match mode {
            ValueKindDisplayMode::ToString => {}
            ValueKindDisplayMode::PrettyPrint(indent_width) => {
                let pad_width = indent_width * depth;
                let pad = " ".repeat(pad_width);
                write!(f, "{pad}")?;
            }
        }
        Ok(())
    }

    // Displays a separator of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_separator(f: &mut fmt::Formatter, mode: &ValueKindDisplayMode, is_last_item: bool) -> fmt::Result {
        match mode {
            ValueKindDisplayMode::ToString => {
                if !is_last_item {
                    write!(f, ", ")?;
                }
            }
            ValueKindDisplayMode::PrettyPrint(_) => {
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
    fn disp_footer(f: &mut fmt::Formatter, mode: &ValueKindDisplayMode, ch: char, depth: usize, is_empty: bool) -> fmt::Result {
        if !is_empty {
            Self::disp_indent(f, mode, depth)?;
        }
        write!(f, "{ch}")
    }

    // Displays `ValueKind::Array`.
    fn disp_array<'a, I>(f: &mut fmt::Formatter, mode: &ValueKindDisplayMode, depth: usize, len: usize, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a Box<Value>>,
    {
        Self::disp_header(f, mode, '[', len == 0)?;
        for (i, v) in iter.enumerate() {
            Self::disp_indent(f, mode, depth + 1)?;
            v.kind.disp(f, mode, depth + 1)?;
            Self::disp_separator(f, mode, i + 1 == len)?;
        }
        Self::disp_footer(f, mode, ']', depth, len == 0)
    }

    // Displays `ValueKind::Object`.
    fn disp_object<'a, I>(f: &mut fmt::Formatter, mode: &ValueKindDisplayMode, depth: usize, len: usize, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a ((String, CodeSpan), Box<Value>)>,
    {
        Self::disp_header(f, mode, '{', len == 0)?;
        for (i, ((k, _), v)) in iter.enumerate() {
            Self::disp_indent(f, mode, depth + 1)?;
            write!(f, r#""{k}": "#)?;
            v.kind.disp(f, mode, depth + 1)?;
            Self::disp_separator(f, mode, i + 1 == len)?;
        }
        Self::disp_footer(f, mode, '}', depth, len == 0)
    }

    // Displays `ValueKind::Literal`.
    fn disp_literal(f: &mut fmt::Formatter, lit: &Literal) -> fmt::Result {
        lit.fmt(f)
    }

    // Displays `ValueKind`.
    fn disp(&self, f: &mut fmt::Formatter, mode: &ValueKindDisplayMode, depth: usize) -> fmt::Result {
        match self {
            Self::Array(vec) => Self::disp_array(f, mode, depth, vec.len(), vec.iter()),
            Self::Object(vec) => Self::disp_object(f, mode, depth, vec.len(), vec.iter()),
            Self::Literal(lit) => Self::disp_literal(f, lit),
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.disp(f, &ValueKindDisplayMode::ToString, 0)
    }
}

impl ValueKind {
    /// Displays [`ValueKind`] via returning the helper struct `ValueKindDisplay`.
    pub const fn display(&self, indent_width: usize) -> ValueKindDisplay<'_> {
        ValueKindDisplay { kind: self, indent_width }
    }
}

/// Helper struct for printing [`ValueKind`].
pub struct ValueKindDisplay<'a> {
    kind: &'a ValueKind,
    indent_width: usize,
}

impl<'a> fmt::Display for ValueKindDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.disp(f, &ValueKindDisplayMode::PrettyPrint(self.indent_width), 0)
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

    /// Displays [`Value`].
    pub const fn display(&self, indent_width: usize) -> ValueKindDisplay<'_> {
        self.kind.display(indent_width)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.kind.fmt(f)
    }
}
