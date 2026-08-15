use std::fmt::{self, Display};

use crate::{CodeSpan, Literal};

/// Represents a kind of JSON value.
#[derive(Debug, PartialEq, Clone)]
#[allow(missing_docs)]
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
#[allow(missing_docs)]
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
        if matches!(self, Self::PrettyPrint(_)) && !is_empty {
            writeln!(f)?;
        }
        Ok(())
    }

    // Displays indent.
    fn disp_indent(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
        if let Self::PrettyPrint(indent_width) = self {
            let pad_width = indent_width * depth;
            let pad = " ".repeat(pad_width);
            write!(f, "{pad}")
        } else {
            Ok(())
        }
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
    fn disp_footer(&self, f: &mut fmt::Formatter, depth: usize, ch: char, is_empty: bool) -> fmt::Result {
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
        self.disp_footer(f, depth, ']', len == 0)
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
        self.disp_footer(f, depth, '}', len == 0)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CodeLocation;

    fn do_display_value_test(kind: ValueKind, expected_tostring: &str, expected_prettyprint: &str) {
        let start = CodeLocation::new(1, 1);
        let end = CodeLocation::new(start.line, start.column + expected_tostring.chars().count());
        let span = CodeSpan::new(start, end);
        let target = Value::new(kind, span);
        assert_eq!(&target.to_string(), expected_tostring);
        assert_eq!(&format!("{}", target.display(1)), expected_prettyprint);
    }

    #[test]
    fn display_value_literal_null() {
        let expected = "null";
        let kind = ValueKind::Literal(Literal::Null);
        do_display_value_test(kind, expected, expected)
    }

    #[test]
    fn display_value_array_empty() {
        let expected = "[]";
        let kind = ValueKind::Array(vec![]);
        do_display_value_test(kind, expected, expected)
    }

    #[test]
    fn display_value_array_single_item() {
        let expected_tostring = "[null]";
        let expected_prettyprint = concat!("[\n", " null\n", "]");
        let loc = CodeLocation::new(1, 1);
        let span = CodeSpan::new(loc, loc);
        let item = Value::new(ValueKind::Literal(Literal::Null), span);
        let kind = ValueKind::Array(vec![Box::new(item)]);
        do_display_value_test(kind, expected_tostring, expected_prettyprint)
    }

    #[test]
    fn display_value_array_multiple_item() {
        let expected_tostring = "[null, null]";
        let expected_prettyprint = concat!("[\n", " null,\n", " null\n", "]");
        let loc = CodeLocation::new(1, 1);
        let span = CodeSpan::new(loc, loc);
        let item = Value::new(ValueKind::Literal(Literal::Null), span);
        let kind = ValueKind::Array(vec![Box::new(item.clone()), Box::new(item)]);
        do_display_value_test(kind, expected_tostring, expected_prettyprint)
    }

    #[test]
    fn display_value_object_empty() {
        let expected = "{}";
        let kind = ValueKind::Object(vec![]);
        do_display_value_test(kind, expected, expected)
    }

    #[test]
    fn display_value_object_single_item() {
        let expected_tostring = r#"{"a": null}"#;
        let expected_prettyprint = concat!("{\n", " \"a\": null\n", "}");
        let name = "a".to_string();
        let loc = CodeLocation::new(1, 1);
        let span = CodeSpan::new(loc, loc);
        let item = Value::new(ValueKind::Literal(Literal::Null), span);
        let kind = ValueKind::Object(vec![((name, span), Box::new(item))]);
        do_display_value_test(kind, expected_tostring, expected_prettyprint)
    }

    #[test]
    fn display_value_object_multiple_item() {
        let expected_tostring = r#"{"a": null, "b": null}"#;
        let expected_prettyprint = concat!("{\n", " \"a\": null,\n", " \"b\": null\n", "}");
        let name0 = "a".to_string();
        let name1 = "b".to_string();
        let loc = CodeLocation::new(1, 1);
        let span = CodeSpan::new(loc, loc);
        let item = Value::new(ValueKind::Literal(Literal::Null), span);
        let kind = ValueKind::Object(vec![((name0, span), Box::new(item.clone())), ((name1, span), Box::new(item))]);
        do_display_value_test(kind, expected_tostring, expected_prettyprint)
    }
}
