// Tests using the test suite from [JSON_checker](https://json.org/JSON_checker/)

use anyhow::{Result, ensure};
use paste::paste;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use toy_json_parser::{BufReadCharsExt as _, Lexer, Parser, TokenKind};

// The test function for `Lexer` and `Parser`.
fn do_parser_test(filename: &str, expected_pasing_result: bool, contains_invalid_token: bool) -> Result<()> {
    let file = File::open(Path::new(filename))?;
    let reader = BufReader::new(file);
    let lexer = Lexer::new(reader.chars());
    let tokens: Vec<_> = lexer.collect();
    ensure!(tokens.iter().any(|token| matches!(token.kind, TokenKind::Invalid(_))) == contains_invalid_token);
    let mut parser = Parser::new(tokens.into_iter());
    let result = parser.parse();
    ensure!(result.is_ok() == expected_pasing_result);
    Ok(())
}

// Macro to define test functions.
macro_rules! generate_parser_tests {
    (
        $prefix:ident, $expected_parsing_result:expr; $(
            $basefilename:ident: $description:ident, $contains_invalid_token:expr
        ),* $(,)?) => {
        $(
            paste! {
                #[test]
                fn [<$prefix _ $basefilename _ $description>]() -> Result<()> {
                    do_parser_test(
                        concat!("tests/json_checker/", stringify!($basefilename), ".json"),
                        $expected_parsing_result,
                        $contains_invalid_token
                    )
                }
            }
        )+
    };
}

// Tests for the patterns that must be accepted.
generate_parser_tests! {parser_accept_pattern, true;
    pass1:  pattern,                   false,
    pass2:  pattern,                   false,
    pass3:  pattern,                   false,
    fail1:  only_one_string_but_regal, false,
    fail18: too_deep_but_regal,        false,
}

// Tests for the patterns that must be rejected.
generate_parser_tests! {parser_reject_pattern, false;
    fail2:  unclosed_array,                     false,
    fail3:  unquoted_key,                       true,
    fail4:  extra_comma_in_array,               false,
    fail5:  double_extra_comma,                 false,
    fail6:  array_missing_value,                false,
    fail7:  comma_after_the_close_array,        false,
    fail8:  extra_close_after_array,            false,
    fail9:  extra_comma_in_object,              false,
    fail10: extra_value_after_the_close_object, false,
    fail11: illegal_expression,                 true,
    fail12: illegal_invocation,                 true,
    fail13: numbers_cannot_have_leading_zeroes, true,
    fail14: numbers_cannot_have_hex,            true,
    fail15: illegal_backslash_escape,           true,
    fail16: naked_backslash,                    true,
    fail17: illegal_backslash_escape,           true,
    fail19: missing_colon,                      false,
    fail20: double_colon,                       false,
    fail21: comma_instead_of_colon,             false,
    fail22: colon_instead_of_comma,             false,
    fail23: bad_value,                          true,
    fail24: single_quote,                       true,
    fail25: tab_character_in_string,            true,
    fail26: tab_character_in_string,            true,
    fail27: line_break_in_string,               true,
    fail28: line_break_in_string,               true,
    fail29: bad_number_0e,                      true,
    fail30: bad_number_0e_plus,                 true,
    fail31: bad_number_0e_plus_minus_1,         true,
    fail32: comma_instead_of_closing_brace,     false,
    fail33: mismatch_closing_delimiter,         false,
}
