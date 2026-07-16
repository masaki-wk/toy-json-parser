// Tests for TokenKind

use toy_json_parser::{Delimiter, Literal, TokenKind};

#[test]
fn len_delimiters() {
    for item in [
        Delimiter::LeftBracket,
        Delimiter::RightBracket,
        Delimiter::LeftBrace,
        Delimiter::RightBrace,
        Delimiter::Colon,
        Delimiter::Comma,
    ] {
        let kind = TokenKind::Delimiter(item);
        assert_eq!(kind.len(), 1);
    }
}

#[test]
fn len_literals() {
    for pair in [
        (Literal::Number("12".to_string()), 2),
        (Literal::String("foo".to_string()), 5),
        (Literal::Boolean(true), 4),
        (Literal::Boolean(false), 5),
        (Literal::Null, 4),
    ] {
        let kind = TokenKind::Literal(pair.0);
        assert_eq!(kind.len(), pair.1);
    }
}

#[test]
fn len_invalid() {
    let pair = (TokenKind::Invalid("foo".to_string()), 3);
    let kind = pair.0;
    assert_eq!(kind.len(), pair.1);
}
