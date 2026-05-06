use katana_ast_lint::rules::LazyCodeOps;
use katana_ast_lint::utils::LinterParserOps;

#[test]
fn library_exposes_rule_api_without_cli() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let syntax = LinterParserOps::parse_file(&path).expect("src/lib.rs must parse");
    let violations = LazyCodeOps::lint(&path, &syntax);
    assert!(violations.is_empty());
}
