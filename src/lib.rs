//! A toy JSON lexer and parser implementation as a study exercise on how to build
//! a lexer and parser.
//!
//! - `Lexer` tokenizes JSON source text into `Token` values.
//! - `Parser` consumes those tokens and builds a tree of `Value` objects.
//! - Both tokens and parsed values carry source position information.
//!
//! Both tokens and parsed values track their source location as `CodeSpan`, which contains:
//!
//! - Line and column numbers (1-indexed)
//! - Start and end positions in the source text
//!
//! The `Parser` returns `ParseError` if the JSON is invalid.
//! Errors include the location where parsing failed, making debugging easier.
//!
//! # Example of `Lexer`
//!
//! ```rust
//! use toy_json_parser::Lexer;
//!
//! let input = r#"{
//!     "foo": null,
//!     "bar": [0, 1]
//! }"#;
//! let mut lexer = Lexer::new(input.chars());
//! for token in lexer {
//!     println!("{:?}: {}", token.kind, token.span);
//! }
//! ```
//!
//! Outputs the following:
//!
//! ```text
//! Delimiter(LeftBrace): [Ln 1, Col 1]..[Ln 1, Col 2]
//! Literal(String("foo")): [Ln 2, Col 5]..[Ln 2, Col 10]
//! Delimiter(Colon): [Ln 2, Col 10]..[Ln 2, Col 11]
//! Literal(Null): [Ln 2, Col 12]..[Ln 2, Col 16]
//! Delimiter(Comma): [Ln 2, Col 16]..[Ln 2, Col 17]
//! Literal(String("bar")): [Ln 3, Col 5]..[Ln 3, Col 10]
//! Delimiter(Colon): [Ln 3, Col 10]..[Ln 3, Col 11]
//! Delimiter(LeftBracket): [Ln 3, Col 12]..[Ln 3, Col 13]
//! Literal(Number("0")): [Ln 3, Col 13]..[Ln 3, Col 14]
//! Delimiter(Comma): [Ln 3, Col 14]..[Ln 3, Col 15]
//! Literal(Number("1")): [Ln 3, Col 16]..[Ln 3, Col 17]
//! Delimiter(RightBracket): [Ln 3, Col 17]..[Ln 3, Col 18]
//! Delimiter(RightBrace): [Ln 4, Col 1]..[Ln 4, Col 2]
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
//!     "bar": [0, 1]
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
//!         1
//!     ]
//! }
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]

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
