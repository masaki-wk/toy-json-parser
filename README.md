# toy-json-parser

A toy JSON lexer and parser implementation as a study exercise on how to build
a lexer and parser.

- `Lexer` tokenizes JSON source text into `Token` values.
- `Parser` consumes those tokens and builds a tree of `Value` objects.
- Both tokens and parsed values carry source position information.

Position tracking makes it easier to understand where tokens and values come
from in the original source text, which is useful for diagnostics, debugging,
and learning parser design.

## Example of `Lexer`

```rust
use toy_json_parser::Lexer;

let input = r#"["foo"]"#;
let mut lexer = Lexer::new(input.chars());
for token in lexer {
    println!("{:?}: {}", token.kind, token.span);
}
```

Outputs the following:

```text
Delimiter(LeftBracket): [Ln 1, Col 1]..[Ln 1, Col 2]
Literal(String("foo")): [Ln 1, Col 2]..[Ln 1, Col 7]
Delimiter(RightBracket): [Ln 1, Col 7]..[Ln 1, Col 8]
```

## Example of `Parser`

```rust
use toy_json_parser::{Lexer, Parser};

let input = r#"{
    "foo": null,
    "bar": [0, 1, 2]
}"#;
let lexer = Lexer::new(input.chars());
let mut parser = Parser::new(lexer);
let value = parser.parse()?;
println!("{:#?}", value);
```

Outputs the following:

```text
{
    "foo": null,
    "bar": [
        0,
        1,
        2
    ]
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
