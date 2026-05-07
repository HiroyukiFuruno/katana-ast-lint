use katana_ast_lint::KatanaAstLint;
use katana_ast_lint::config::{KalConfig, RuleConfig};
use katana_ast_lint::rules::{LazyCodeOps, RULE_CATALOG};
use katana_ast_lint::utils::LinterParserOps;
use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};

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

#[test]
fn with_config_lints_current_directory_when_roots_are_not_configured() {
    let mut fixture = tempfile::Builder::new()
        .prefix("kal_review_fixture")
        .suffix(".rs")
        .tempfile_in(env!("CARGO_MANIFEST_DIR"))
        .expect("fixture file must be created");

    writeln!(fixture, "pub fn fixture() {{ todo!() }}").expect("fixture must be written");
    fixture.flush().expect("fixture must be flushed");
    let fixture_path = fixture.path().to_path_buf();
    let config = KalConfig {
        rules: enabled_rules(&["lazy-code"]),
        ..KalConfig::default()
    };

    let violations = KatanaAstLint::with_config(config).violations();

    assert!(
        violations.iter().any(|it| it.file == fixture_path),
        "with_config must fall back to the current directory"
    );
}

#[test]
fn one_line_runner_keeps_function_length_default_threshold() {
    let project = TempProject::new();
    let source = project.source_file("src/lib.rs");
    project.write_source(&source, &function_with_lines(35));
    let config = KalConfig {
        source_roots: vec![project.src_dir()],
        rules: enabled_rules(&["function-length"]),
        ..KalConfig::default()
    };

    let violations = KatanaAstLint::with_config(config).violations();

    assert!(
        violations.iter().any(|it| it.file == source),
        "31-50 line functions must keep failing by default"
    );
}

#[test]
fn one_line_runner_keeps_nesting_depth_default_threshold() {
    let project = TempProject::new();
    let source = project.source_file("src/lib.rs");
    project.write_source(
        &source,
        r#"
fn nested() {
    if true {
        if true {
            if true {
                if true {}
            }
        }
    }
}
"#,
    );
    let config = KalConfig {
        source_roots: vec![project.src_dir()],
        rules: enabled_rules(&["nesting-depth"]),
        ..KalConfig::default()
    };

    let violations = KatanaAstLint::with_config(config).violations();

    assert!(
        violations.iter().any(|it| it.file == source),
        "depth 4 must keep failing by default"
    );
}

#[test]
fn one_line_runner_keeps_file_length_test_default_threshold() {
    let project = TempProject::new();
    let source = project.source_file("consumer_tests.rs");
    project.write_source(&source, &blank_lines(250));
    let config = KalConfig {
        source_roots: vec![project.root_dir()],
        rules: enabled_rules(&["file-length"]),
        ..KalConfig::default()
    };

    let violations = KatanaAstLint::with_config(config).violations();

    assert!(
        violations.iter().all(|it| it.file != source),
        "root-level *_tests.rs files must keep the 300-line default"
    );
}

#[test]
fn assert_clean_reports_multiple_rule_failures_together() {
    let project = TempProject::new();
    let source = project.source_file("src/lib.rs");
    project.write_source(&source, "pub fn fixture() { todo!() }\n");
    let config = KalConfig {
        source_roots: vec![project.src_dir()],
        rules: enabled_rules(&["lazy-code", "pub-free-fn"]),
        ..KalConfig::default()
    };

    let result = std::panic::catch_unwind(|| {
        KatanaAstLint::with_config(config).assert_clean();
    });
    let message = panic_message(result.expect_err("assert_clean must fail"));

    assert!(message.contains("Rule: lazy-code"));
    assert!(message.contains("Rule: pub-free-fn"));
}

struct TempProject {
    dir: tempfile::TempDir,
}

impl TempProject {
    fn new() -> Self {
        Self {
            dir: tempfile::tempdir().expect("tempdir must be created"),
        }
    }

    fn root_dir(&self) -> PathBuf {
        self.dir.path().to_path_buf()
    }

    fn src_dir(&self) -> PathBuf {
        self.source_file("src")
    }

    fn source_file(&self, path: &str) -> PathBuf {
        self.dir.path().join(path)
    }

    fn write_source(&self, path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("source directory must be created");
        }
        std::fs::write(path, content).expect("source file must be written");
    }
}

fn enabled_rules(rule_ids: &[&str]) -> HashMap<String, RuleConfig> {
    RULE_CATALOG
        .iter()
        .map(|it| {
            let enabled = rule_ids.contains(&it.id);
            let config = RuleConfig {
                enabled: Some(enabled),
                ..RuleConfig::default()
            };
            (it.id.to_string(), config)
        })
        .collect()
}

fn function_with_lines(line_count: usize) -> String {
    let mut source = String::from("fn fixture() {\n");
    for _ in 0..line_count {
        source.push_str("    let value = 1;\n");
    }
    source.push_str("}\n");
    source
}

fn blank_lines(line_count: usize) -> String {
    "\n".repeat(line_count)
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    if let Some(message) = payload.downcast_ref::<&str>() {
        return (*message).to_string();
    }
    String::new()
}
