use katana_ast_lint::rules::{LazyCodeOps, RULE_CATALOG};
use katana_ast_lint::utils::LinterParserOps;

#[test]
fn library_exposes_rule_api_without_cli() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/lib.rs");
    let syntax = LinterParserOps::parse_file(&path).expect("src/lib.rs must parse");
    let violations = LazyCodeOps::lint(&path, &syntax);
    assert!(violations.is_empty());
}

#[test]
fn all_public_rules_are_registered_in_catalog() {
    // This list should be manually updated when new rules are added to the public API
    // to ensure they are also registered in the RULE_CATALOG.
    let expected_rules = vec![
        "comment-style",
        "conditional-frame",
        "error-first",
        "frame-stroke",
        "horizontal-layout",
        "icon-button-fill",
        "lazy-code",
        "magic-numbers",
        "markdown-sandbox",
        "min-rect-sizing",
        "performance",
        "process-command",
        "prohibited-attributes",
        "prohibited-types",
        "scrollarea-inner-rect-leak",
        "file-length",
        "function-length",
        "nesting-depth",
        "pub-free-fn",
        "type-separation",
        "i18n",
        "icon",
        "locales",
    ];

    for id in expected_rules {
        assert!(
            RULE_CATALOG.iter().any(|r| r.id == id),
            "Rule '{}' is missing from RULE_CATALOG",
            id
        );
    }
}
