# toy-json-parser

[![GitHub](https://img.shields.io/badge/GitHub-masaki--wk/toy--json--parser-informational?logo=github)](https://github.com/masaki-wk/toy-json-parser)
[![crates.io](https://img.shields.io/crates/v/toy-json-parser?logo=rust)](https://crates.io/crates/toy-json-parser)
[![Docs.rs](https://img.shields.io/badge/Docs.rs-toy--json--parser-informational?logo=docsdotrs)](https://docs.rs/toy-json-parser/)
[![CI Status](https://github.com/masaki-wk/toy-json-parser/actions/workflows/ci.yml/badge.svg)](https://github.com/masaki-wk/toy-json-parser/actions/workflows/ci.yml)
[![Docs Status](https://img.shields.io/docsrs/toy-json-parser?logo=docsdotrs)](https://docs.rs/crate/toy-json-parser/latest/builds)

A toy JSON lexer and parser developed as a learning project on how to build
lexers and parsers.

- `Lexer` tokenizes JSON source text into `Token`s.
- `Parser` consumes those tokens and builds a tree of `Value`s.

Both `Token`s and `Value`s track their spans in the source text. Each span
contains a start location and an end location, and each location includes line
and column numbers.

`Lexer` reports lexical errors, and `Parser` reports lexical or syntactic
errors. Errors include the location where lexing or parsing failed, making
problems easier to diagnose.

## Examples

The following example code of `Lexer` tokenizes a JSON string into tokens and
prints their kinds and spans.

```rust
use toy_json_parser::Lexer;

let input = r#"{
    "foo": null,
    "bar": [0, 1]
}"#;
let mut lexer = Lexer::new(input.chars());
for result in lexer {
    let token = result.unwrap();
    println!("{:?}: {}", token.kind, token.span.start);
}
```

This outputs the following:

```text
Delimiter(LeftBrace): [Ln 1, Col 1]
Literal(String("foo")): [Ln 2, Col 5]
Delimiter(Colon): [Ln 2, Col 10]
Literal(Null): [Ln 2, Col 12]
Delimiter(Comma): [Ln 2, Col 16]
Literal(String("bar")): [Ln 3, Col 5]
Delimiter(Colon): [Ln 3, Col 10]
Delimiter(LeftBracket): [Ln 3, Col 12]
Literal(Number("0")): [Ln 3, Col 13]
Delimiter(Comma): [Ln 3, Col 14]
Literal(Number("1")): [Ln 3, Col 16]
Delimiter(RightBracket): [Ln 3, Col 17]
Delimiter(RightBrace): [Ln 4, Col 1]
```

The following example code of `Lexer` and `Parser` parses a JSON string into
a value and prints it.

```rust
use toy_json_parser::{Lexer, Parser};

let input = r#"{
    "foo": null,
    "bar": [0, 1]
}"#;
let lexer = Lexer::new(input.chars());
let mut parser = Parser::new(lexer);
let value = parser.parse()?;
println!("{}", value.display(4));
```

This outputs the following:

```text
{
    "foo": null,
    "bar": [
        0,
        1
    ]
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
