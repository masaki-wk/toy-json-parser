//! A toy JSON lexer and parser implementation as a study exercise on how to build
//! a lexer and parser.
//!
//! - `Lexer` tokenizes JSON source text into `Token` values.
//! - `Parser` consumes those tokens and builds a tree of `Value` objects.
//! - Both tokens and parsed values carry source position information.
//!
//! Position tracking makes it easier to understand where tokens and values come
//! from in the original source text, which is useful for diagnostics, debugging,
//! and learning parser design.
//!
//! # Example of `Lexer`
//!
//! ```rust
//! use toy_json_parser::Lexer;
//!
//! let input = r#"["foo"]"#;
//! let mut lexer = Lexer::new(input.chars());
//! for token in lexer {
//!     println!("{:?}: {}", token.kind, token.span);
//! }
//! ```
//!
//! Outputs the following:
//!
//! ```text
//! Delimiter(LeftBracket): [Ln 1, Col 1]..[Ln 1, Col 2]
//! Literal(String("foo")): [Ln 1, Col 2]..[Ln 1, Col 7]
//! Delimiter(RightBracket): [Ln 1, Col 7]..[Ln 1, Col 8]
//! ```
//!
//! # Example of `Parser`
//!
//! ```rust
//! use toy_json_parser::{Lexer, Parser};
//! # use toy_json_parser::ParseError;
//!
//! # fn test() -> Result<(), ParseError> {
//! let input = r#"{
//!     "foo": null,
//!     "bar": [0, 1, 2]
//! }"#;
//! let lexer = Lexer::new(input.chars());
//! let mut parser = Parser::new(lexer);
//! let value = parser.parse()?;
//! println!("{}", value.display(4));
//! # Ok(())
//! # }
//! ```
//!
//! Outputs the following:
//!
//! ```text
//! {
//!     "foo": null,
//!     "bar": [
//!         0,
//!         1,
//!         2
//!     ]
//! }
//! ```

mod code_location;
pub use code_location::CodeLocation;

mod code_span;
pub use code_span::CodeSpan;

mod iterator_location_ext;
pub use iterator_location_ext::IteratorLocationExt;
pub use iterator_location_ext::LocatedIterator;

mod token;
pub use token::Delimiter;
pub use token::Literal;
pub use token::Token;
pub use token::TokenKind;

mod lexer;
pub use lexer::Lexer;

mod value;
pub use value::Value;
pub use value::ValueDisplay;
pub use value::ValueKind;

mod parser;
pub use parser::ParseError;
pub use parser::Parser;

mod bufread_chars_ext;
pub use bufread_chars_ext::BufReadChars;
pub use bufread_chars_ext::BufReadCharsExt;
