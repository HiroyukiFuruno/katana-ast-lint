use super::*;
use std::path::Path;

#[test]
fn is_allowed_string_allows_multiplication_sign() {
    assert!(LinterParserOps::is_allowed_string("×"));
}

#[test]
fn is_allowed_string_allows_common_ui_shorthands() {
    assert!(LinterParserOps::is_allowed_string("x"));
    assert!(LinterParserOps::is_allowed_string("X"));
}

#[test]
fn is_allowed_string_denies_normal_words() {
    assert!(!LinterParserOps::is_allowed_string("Hello"));
    assert!(!LinterParserOps::is_allowed_string("Save"));
    assert!(!LinterParserOps::is_allowed_string("a"));
}

#[test]
fn is_allowed_string_allows_symbols_and_numbers() {
    assert!(LinterParserOps::is_allowed_string("123"));
    assert!(LinterParserOps::is_allowed_string("1.0"));
    assert!(LinterParserOps::is_allowed_string("(!)"));
    assert!(LinterParserOps::is_allowed_string("🔄"));
}

#[test]
fn parse_file_handles_missing_file() {
    let result = LinterParserOps::parse_file(Path::new("missing_file_random_123.rs"));
    let Err(violations) = result else {
        panic!("Expected error for missing file");
    };
    assert!(violations[0].message.contains("Rust file read error"));
}

#[test]
fn parse_file_handles_invalid_syntax() {
    let tmp = tempfile::NamedTempFile::with_suffix(".rs").unwrap();
    std::fs::write(tmp.path(), "invalid rust code").unwrap();
    let result = LinterParserOps::parse_file(tmp.path());
    let Err(violations) = result else {
        panic!("Expected error for invalid syntax");
    };
    assert!(violations[0].message.contains("Syntax parse error"));
}
