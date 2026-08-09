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
#[derive(Debug, Clone, Copy)]
enum ValueKindDisplayMode {
    ToString,
    PrettyPrint(usize),
}

impl ValueKind {
    // Returns a padding string for `disp()`.
    fn padding(mode: ValueKindDisplayMode, depth: usize) -> String {
        match mode {
            ValueKindDisplayMode::ToString => String::new(),
            ValueKindDisplayMode::PrettyPrint(indent_width) => {
                let pad_width = indent_width * depth;
                " ".repeat(pad_width)
            }
        }
    }

    // Displays a header of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_header(ch: char, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{ch}")
    }

    // Displays a separator of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_separator(mode: ValueKindDisplayMode, pad: &str, is_first_item: bool, f: &mut fmt::Formatter) -> fmt::Result {
        match mode {
            ValueKindDisplayMode::ToString => {
                if !is_first_item {
                    write!(f, ", ")?;
                }
            }
            ValueKindDisplayMode::PrettyPrint(_) => {
                if is_first_item {
                    writeln!(f)?;
                } else {
                    writeln!(f, ",")?;
                }
                write!(f, "{pad}")?;
            }
        }
        Ok(())
    }

    // Displays the footer of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_footer(mode: ValueKindDisplayMode, ch: char, pad: &str, is_empty: bool, f: &mut fmt::Formatter) -> fmt::Result {
        match mode {
            ValueKindDisplayMode::ToString => {}
            ValueKindDisplayMode::PrettyPrint(_) => {
                if !is_empty {
                    writeln!(f)?;
                    write!(f, "{pad}")?;
                }
            }
        }
        write!(f, "{ch}")
    }

    // Displays `ValueKind::Array`.
    fn disp_array<'a, I>(mode: ValueKindDisplayMode, current_depth: usize, f: &mut fmt::Formatter, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a Box<Value>>,
    {
        let pad_curr = Self::padding(mode, current_depth);
        let pad_sub = Self::padding(mode, current_depth + 1);
        let mut is_empty = true;
        Self::disp_header('[', f)?;
        for (i, v) in iter.enumerate() {
            Self::disp_separator(mode, &pad_sub, i == 0, f)?;
            v.kind.disp(mode, current_depth + 1, f)?;
            is_empty = false;
        }
        Self::disp_footer(mode, ']', &pad_curr, is_empty, f)
    }

    // Displays `ValueKind::Object`.
    fn disp_object<'a, I>(mode: ValueKindDisplayMode, current_depth: usize, f: &mut fmt::Formatter, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a ((String, CodeSpan), Box<Value>)>,
    {
        let pad_curr = Self::padding(mode, current_depth);
        let pad_sub = Self::padding(mode, current_depth + 1);
        let mut is_empty = true;
        Self::disp_header('{', f)?;
        for (i, ((k, _), v)) in iter.enumerate() {
            Self::disp_separator(mode, &pad_sub, i == 0, f)?;
            write!(f, r#""{k}": "#)?;
            v.kind.disp(mode, current_depth + 1, f)?;
            is_empty = false;
        }
        Self::disp_footer(mode, '}', &pad_curr, is_empty, f)
    }

    // Displays `ValueKind::Literal`.
    fn disp_literal(f: &mut fmt::Formatter, lit: &Literal) -> fmt::Result {
        lit.fmt(f)
    }

    // Displays `ValueKind`.
    fn disp(&self, mode: ValueKindDisplayMode, current_depth: usize, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Array(vec) => Self::disp_array(mode, current_depth, f, vec.iter()),
            Self::Object(vec) => Self::disp_object(mode, current_depth, f, vec.iter()),
            Self::Literal(lit) => Self::disp_literal(f, lit),
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.disp(ValueKindDisplayMode::ToString, 0, f)
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
        self.kind.disp(ValueKindDisplayMode::PrettyPrint(self.indent_width), 0, f)
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
