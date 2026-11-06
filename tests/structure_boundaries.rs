use katana_ast_lint::config::RuleConfig;
use katana_ast_lint::rules::{FileLengthOps, FunctionLengthOps};
use std::path::Path;

fn threshold(lines: usize) -> RuleConfig {
    RuleConfig {
        threshold: Some(lines),
        ..RuleConfig::default()
    }
}

#[test]
fn function_length_uses_brace_lines_and_reports_the_function_name() {
    let source = format!(
        "fn boundary() {{\n{at_limit}}}\nimpl Widget {{\n    fn over_limit() {{\n{over_limit}    }}\n}}\n",
        at_limit = "\n".repeat(29),
        over_limit = "\n".repeat(30),
    );
    let syntax = syn::parse_file(&source).unwrap();
    let path = Path::new("lib.rs");
    let violations = FunctionLengthOps::lint(path, &syntax);

    assert_eq!(violations.len(), 1);
    assert_eq!((violations[0].line, violations[0].column), (33, 8));
    assert!(violations[0].message.contains("over_limit"));
    assert!(violations[0].message.contains("current: 31"));
    assert!(FunctionLengthOps::lint_with_config(path, &syntax, &threshold(31)).is_empty());
}

fn lint_file_length(source: &str, lines: usize) -> Vec<katana_ast_lint::Violation> {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("lib.rs");
    std::fs::write(&path, source).unwrap();
    let syntax = syn::parse_file(source).unwrap();
    FileLengthOps::lint_with_config(&path, &syntax, &threshold(lines))
}

#[test]
fn file_length_counts_production_after_external_and_inline_test_modules() {
    let source = concat!(
        "fn before() {}\n",
        "#[cfg(test)]\nmod external;\n",
        "#[cfg(\n    test\n)]\npub(crate) mod inline {\n",
        "    fn helper() {}\n",
        "}\n",
        "fn after() {}\n",
    );
    assert!(lint_file_length(source, 2).is_empty());

    let violations = lint_file_length(source, 1);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].message.contains("current: 2"));
}

#[test]
fn file_length_preserves_shared_lines_and_ignores_cfg_text_inside_strings() {
    let source = concat!(
        "mod production {\n",
        "    fn before() {} #[cfg(test)] mod test_helpers {\n",
        "        fn helper() {}\n",
        "    } fn after() {}\n",
        "}\n",
        "const TEXT: &str = r#\"\n",
        "#[cfg(test)]\nmod not_a_module;\n",
        "\"#;\n",
    );

    let violations = lint_file_length(source, 7);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].message.contains("current: 8"));
    assert!(lint_file_length(source, 8).is_empty());
}
