// Tests with the test suite from [JSON_checker](https://json.org/JSON_checker/)

use anyhow::{Result, ensure};
use std::fs::File;
use std::io::BufReader;
use toy_json_parser::{BufReadCharsExt as _, Lexer, Parser, TokenKind};

fn do_parser_test(filename: &str, expected_pasing_result: bool, contains_invalid_token: bool) -> Result<()> {
    let path = format!("tests/json_checker/{filename}");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lexer = Lexer::new(reader.chars());
    let tokens: Vec<_> = lexer.collect();
    ensure!(tokens.iter().any(|token| matches!(token.kind, TokenKind::Invalid(_))) == contains_invalid_token);
    let mut parser = Parser::new(tokens.into_iter());
    let result = parser.parse();
    ensure!(result.is_ok() == expected_pasing_result);
    Ok(())
}

macro_rules! generate_parser_accept_pattern_tests {
    ($($testname:ident: $filename:expr, $contains_invalid_token:expr),* $(,)?) => {
        $(
            #[test]
            fn $testname() -> Result<()> {
                do_parser_test($filename, true, $contains_invalid_token)
            }
        )+
    };
}

macro_rules! generate_parser_deny_pattern_tests {
    ($($testname:ident: $filename:expr, $contains_invalid_token:expr),* $(,)?) => {
        $(
            #[test]
            fn $testname() -> Result<()> {
                do_parser_test($filename, false, $contains_invalid_token)
            }
        )+
    };
}

#[rustfmt::skip]
generate_parser_accept_pattern_tests! {
    parser_accept_pattern_pass1:                 "pass1.json",  false,
    parser_accept_pattern_pass2:                 "pass2.json",  false,
    parser_accept_pattern_pass3:                 "pass3.json",  false,
    parser_accept_pattern_fail1_only_one_string: "fail1.json",  false,
    parser_accept_pattern_fail18_too_deep:       "fail18.json", false,
}

#[rustfmt::skip]
generate_parser_deny_pattern_tests! {
    parser_deny_pattern_fail2_unclosed_array:                      "fail2.json",  false,
    parser_deny_pattern_fail3_unquoted_key:                        "fail3.json",  true,
    parser_deny_pattern_fail4_extra_comma_in_array:                "fail4.json",  false,
    parser_deny_pattern_fail5_double_extra_comma:                  "fail5.json",  false,
    parser_deny_pattern_fail6_array_missing_value:                 "fail6.json",  false,
    parser_deny_pattern_fail7_comma_after_the_close_array:         "fail7.json",  false,
    parser_deny_pattern_fail8_extra_close_after_array:             "fail8.json",  false,
    parser_deny_pattern_fail9_extra_comma_in_object:               "fail9.json",  false,
    parser_deny_pattern_fail10_extra_value_after_the_close_object: "fail10.json", false,
    parser_deny_pattern_fail11_illegal_expression:                 "fail11.json", true,
    parser_deny_pattern_fail12_illegal_invocation:                 "fail12.json", true,
    parser_deny_pattern_fail13_numbers_cannot_have_leading_zeroes: "fail13.json", true,
    parser_deny_pattern_fail14_numbers_cannot_have_hex:            "fail14.json", true,
    parser_deny_pattern_fail15_illegal_backslash_escape:           "fail15.json", true,
    parser_deny_pattern_fail16_naked_backslash:                    "fail16.json", true,
    parser_deny_pattern_fail17_illegal_backslash_escape:           "fail17.json", true,
    parser_deny_pattern_fail19_missing_colon:                      "fail19.json", false,
    parser_deny_pattern_fail20_double_colon:                       "fail20.json", false,
    parser_deny_pattern_fail21_comma_instead_of_colon:             "fail21.json", false,
    parser_deny_pattern_fail22_colon_instead_of_comma:             "fail22.json", false,
    parser_deny_pattern_fail23_bad_value:                          "fail23.json", true,
    parser_deny_pattern_fail24_single_quote:                       "fail24.json", true,
    parser_deny_pattern_fail25_tab_character_in_string:            "fail25.json", true,
    parser_deny_pattern_fail26_tab_character_in_string:            "fail26.json", true,
    parser_deny_pattern_fail27_line_break_in_string:               "fail27.json", true,
    parser_deny_pattern_fail28_line_break_in_string:               "fail28.json", true,
    parser_deny_pattern_fail29_bad_number_0e:                      "fail29.json", true,
    parser_deny_pattern_fail30_bad_number_0e_plus:                 "fail30.json", true,
    parser_deny_pattern_fail31_bad_number_0e_plus_minus_1:         "fail31.json", true,
    parser_deny_pattern_fail32_comma_instead_of_closing_brace:     "fail32.json", false,
    parser_deny_pattern_fail33_mismatch_closing_delimiter:         "fail33.json", false,
}
