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
    pub const fn display(&self, indent_width: usize, show_span: bool) -> ValueDisplay<'_> {
        ValueDisplay::new(ValueDisplayMode::PrettyPrint(indent_width, show_span), self, None)
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = ValueDisplay::new(ValueDisplayMode::ToString, self, None);
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
    pub const fn display(&self, indent_width: usize, show_span: bool) -> ValueDisplay<'_> {
        ValueDisplay::new(ValueDisplayMode::PrettyPrint(indent_width, show_span), &self.kind, Some(&self.span))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let disp = ValueDisplay::new(ValueDisplayMode::ToString, &self.kind, Some(&self.span));
        disp.fmt(f)
    }
}

// Helper enum for `ValueDisplay`
#[derive(Debug, Clone)]
enum ValueDisplayMode {
    ToString,
    PrettyPrint(usize, bool),
}

/// Helper struct for printing [`Value`].
pub struct ValueDisplay<'a> {
    mode: ValueDisplayMode,
    kind: &'a ValueKind,
    span: Option<&'a CodeSpan>,
}

impl<'a> ValueDisplay<'a> {
    // Creates a new `ValueDisplay`.
    const fn new(mode: ValueDisplayMode, kind: &'a ValueKind, span: Option<&'a CodeSpan>) -> Self {
        Self { mode, kind, span }
    }

    // Displays a header of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_header(f: &mut fmt::Formatter, mode: &ValueDisplayMode, span: Option<&'a CodeSpan>, ch: char, is_empty: bool) -> fmt::Result {
        write!(f, "{ch}")?;
        match mode {
            ValueDisplayMode::ToString => {}
            ValueDisplayMode::PrettyPrint(_, show_span) => {
                if !is_empty {
                    if let Some(span) = span
                        && *show_span
                    {
                        let start = span.start;
                        write!(f, " {start}")?;
                    }
                    writeln!(f)?;
                }
            }
        }
        Ok(())
    }

    // Displays indent.
    fn disp_indent(f: &mut fmt::Formatter, mode: &ValueDisplayMode, depth: usize) -> fmt::Result {
        match mode {
            ValueDisplayMode::ToString => {}
            ValueDisplayMode::PrettyPrint(indent_width, _) => {
                let pad_width = indent_width * depth;
                let pad = " ".repeat(pad_width);
                write!(f, "{pad}")?;
            }
        }
        Ok(())
    }

    // Displays suffix of an item of `ValueKind::Array` or `ValueKind::Object`.
    fn disp_item_suffix(f: &mut fmt::Formatter, mode: &ValueDisplayMode, is_last_item: bool) -> fmt::Result {
        match mode {
            ValueDisplayMode::ToString => {
                if !is_last_item {
                    write!(f, ", ")?;
                }
            }
            ValueDisplayMode::PrettyPrint(_, _) => {
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
    fn disp_footer(f: &mut fmt::Formatter, mode: &ValueDisplayMode, ch: char, depth: usize, is_empty: bool) -> fmt::Result {
        if !is_empty {
            Self::disp_indent(f, mode, depth)?;
        }
        write!(f, "{ch}")
    }

    // Displays `ValueKind::Array`.
    fn disp_array<I>(f: &mut fmt::Formatter, mode: &'a ValueDisplayMode, span: Option<&'a CodeSpan>, depth: usize, len: usize, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a Box<Value>>,
    {
        Self::disp_header(f, mode, span, '[', len == 0)?;
        for (i, v) in iter.enumerate() {
            Self::disp_indent(f, mode, depth + 1)?;
            let sub = ValueDisplay::new(mode.clone(), &v.kind, Some(&v.span));
            sub.disp(f, depth + 1)?;
            Self::disp_item_suffix(f, mode, i + 1 == len)?;
        }
        Self::disp_footer(f, mode, ']', depth, len == 0)
    }

    // Displays `ValueKind::Object`.
    fn disp_object<I>(f: &mut fmt::Formatter, mode: &'a ValueDisplayMode, span: Option<&'a CodeSpan>, depth: usize, len: usize, iter: I) -> fmt::Result
    where
        I: Iterator<Item = &'a ((String, CodeSpan), Box<Value>)>,
    {
        Self::disp_header(f, mode, span, '{', len == 0)?;
        for (i, ((k, _), v)) in iter.enumerate() {
            Self::disp_indent(f, mode, depth + 1)?;
            write!(f, r#""{k}": "#)?;
            let sub = ValueDisplay::new(mode.clone(), &v.kind, Some(&v.span));
            sub.disp(f, depth + 1)?;
            Self::disp_item_suffix(f, mode, i + 1 == len)?;
        }
        Self::disp_footer(f, mode, '}', depth, len == 0)
    }

    // Displays `ValueKind::Literal`.
    fn disp_literal(f: &mut fmt::Formatter, lit: &Literal) -> fmt::Result {
        lit.fmt(f)
    }

    // Displays `ValueKind`.
    fn disp<'b>(&'b self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result
    where
        'b: 'a,
    {
        match self.kind {
            ValueKind::Array(vec) => Self::disp_array(f, &self.mode, self.span, depth, vec.len(), vec.iter()),
            ValueKind::Object(vec) => Self::disp_object(f, &self.mode, self.span, depth, vec.len(), vec.iter()),
            ValueKind::Literal(lit) => Self::disp_literal(f, lit),
        }
    }
}

impl<'a> fmt::Display for ValueDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.disp(f, 0)
    }
}
