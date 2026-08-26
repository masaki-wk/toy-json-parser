# toy-json-parser

A toy JSON lexer and parser developed as a learning project.

- `Lexer` tokenizes JSON source text into tokens.
- `Parser` consumes those tokens and builds a tree of JSON values.

Both tokens and parsed values track their spans in the source text. Each span
contains start and end positions, and each position includes 1-indexed line and
column numbers.

`Lexer` reports lexical errors, and `Parser` reports lexical or syntactic
errors. Errors include the location where lexing or parsing failed, making
problems easier to diagnose.

## Examples

The example code of `Lexer` shown below:

```rust
use toy_json_parser::Lexer;

let input = r#"{
    "foo": null,
    "bar": [0, 1]
}"#;
let mut lexer = Lexer::new(input.chars());
for result in lexer {
    let token = result.unwrap();
    println!("{:?}: {}", token.kind, token.span);
}
```

Outputs the following:

```
Delimiter(LeftBrace): [Ln 1, Col 1]..[Ln 1, Col 2]
Literal(String("foo")): [Ln 2, Col 5]..[Ln 2, Col 10]
Delimiter(Colon): [Ln 2, Col 10]..[Ln 2, Col 11]
Literal(Null): [Ln 2, Col 12]..[Ln 2, Col 16]
Delimiter(Comma): [Ln 2, Col 16]..[Ln 2, Col 17]
Literal(String("bar")): [Ln 3, Col 5]..[Ln 3, Col 10]
Delimiter(Colon): [Ln 3, Col 10]..[Ln 3, Col 11]
Delimiter(LeftBracket): [Ln 3, Col 12]..[Ln 3, Col 13]
Literal(Number("0")): [Ln 3, Col 13]..[Ln 3, Col 14]
Delimiter(Comma): [Ln 3, Col 14]..[Ln 3, Col 15]
Literal(Number("1")): [Ln 3, Col 16]..[Ln 3, Col 17]
Delimiter(RightBracket): [Ln 3, Col 17]..[Ln 3, Col 18]
Delimiter(RightBrace): [Ln 4, Col 1]..[Ln 4, Col 2]
```

The example code of `Lexer` and `Parser` shown below:

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

Outputs the following:

```
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
