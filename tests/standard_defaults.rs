use katana_ast_lint::KatanaAstLint;
use katana_ast_lint::config::{KalConfig, RuleConfig};
use katana_ast_lint::rules::{RULE_CATALOG, RuleDefinition};
use std::path::{Path, PathBuf};

struct TempRustProject {
    dir: tempfile::TempDir,
}

impl TempRustProject {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir must be created");
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).expect("src directory must be created");
        Self { dir }
    }

    fn src(&self) -> PathBuf {
        self.dir.path().join("src")
    }

    fn write_rs(&self, relative_path: &str, line_count: usize) {
        let path = self.src().join(relative_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("parent directory must be created");
        }
        std::fs::write(path, repeated_comment_lines(line_count))
            .expect("rust source must be written");
    }
}

fn repeated_comment_lines(line_count: usize) -> String {
    let mut content = String::new();
    for line_number in 1..=line_count {
        content.push_str(&format!("// standard default test line {line_number}\n"));
    }
    content
}

fn catalog_rule(rule_id: &str) -> &'static RuleDefinition {
    RULE_CATALOG
        .iter()
        .find(|it| it.id == rule_id)
        .unwrap_or_else(|| panic!("rule {rule_id} must be registered"))
}

fn config_with_only_file_length(source_root: &Path) -> KalConfig {
    let mut config = KalConfig {
        source_roots: vec![source_root.to_path_buf()],
        ..Default::default()
    };

    for rule in RULE_CATALOG.iter() {
        config.rules.insert(
            rule.id.to_string(),
            RuleConfig {
                enabled: Some(rule.id == "file-length"),
                ..Default::default()
            },
        );
    }

    config
}

#[test]
fn catalog_structure_defaults_match_katana_baseline() {
    assert_eq!(catalog_rule("function-length").default_threshold, Some(30));
    assert_eq!(catalog_rule("nesting-depth").default_threshold, Some(3));
    assert_eq!(catalog_rule("file-length").default_threshold, None);
}

#[test]
fn standard_file_length_keeps_source_boundary_at_200_lines() {
    let project = TempRustProject::new();
    project.write_rs("large.rs", 201);
    let linter = KatanaAstLint::with_config(config_with_only_file_length(&project.src()));

    let violations = linter.violations();

    assert_eq!(violations.len(), 1);
    assert!(violations[0].message.contains("200-line limit"));
}

#[test]
fn standard_file_length_allows_300_line_test_modules_without_threshold_config() {
    let project = TempRustProject::new();
    project.write_rs("large_tests.rs", 300);
    let linter = KatanaAstLint::with_config(config_with_only_file_length(&project.src()));

    let violations = linter.violations();

    assert!(violations.is_empty());
}
