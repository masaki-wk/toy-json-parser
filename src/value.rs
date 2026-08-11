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
    fn disp_header(mode: &ValueKindDisplayMode, ch: char, is_empty: bool, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn disp_indent(mode: &ValueKindDisplayMode, depth: usize, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn disp_separator(mode: &ValueKindDisplayMode, is_last_item: bool, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn disp_footer(mode: &ValueKindDisplayMode, ch: char, depth: usize, is_empty: bool, f: &mut fmt::Formatter) -> fmt::Result {
        if !is_empty {
            Self::disp_indent(mode, depth, f)?;
        }
        write!(f, "{ch}")
    }

    // Displays `ValueKind::Array`.
    fn disp_array<'a, I>(mode: &ValueKindDisplayMode, depth: usize, len: usize, f: &mut fmt::Formatter, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a Box<Value>>,
    {
        Self::disp_header(mode, '[', len == 0, f)?;
        for (i, v) in iter.enumerate() {
            Self::disp_indent(mode, depth + 1, f)?;
            v.kind.disp(mode, depth + 1, f)?;
            Self::disp_separator(mode, i + 1 == len, f)?;
        }
        Self::disp_footer(mode, ']', depth, len == 0, f)
    }

    // Displays `ValueKind::Object`.
    fn disp_object<'a, I>(mode: &ValueKindDisplayMode, depth: usize, len: usize, f: &mut fmt::Formatter, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a ((String, CodeSpan), Box<Value>)>,
    {
        Self::disp_header(mode, '{', len == 0, f)?;
        for (i, ((k, _), v)) in iter.enumerate() {
            Self::disp_indent(mode, depth + 1, f)?;
            write!(f, r#""{k}": "#)?;
            v.kind.disp(mode, depth + 1, f)?;
            Self::disp_separator(mode, i + 1 == len, f)?;
        }
        Self::disp_footer(mode, '}', depth, len == 0, f)
    }

    // Displays `ValueKind::Literal`.
    fn disp_literal(f: &mut fmt::Formatter, lit: &Literal) -> fmt::Result {
        lit.fmt(f)
    }

    // Displays `ValueKind`.
    fn disp(&self, mode: &ValueKindDisplayMode, depth: usize, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Array(vec) => Self::disp_array(mode, depth, vec.len(), f, vec.iter()),
            Self::Object(vec) => Self::disp_object(mode, depth, vec.len(), f, vec.iter()),
            Self::Literal(lit) => Self::disp_literal(f, lit),
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.disp(&ValueKindDisplayMode::ToString, 0, f)
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
        self.kind.disp(&ValueKindDisplayMode::PrettyPrint(self.indent_width), 0, f)
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
