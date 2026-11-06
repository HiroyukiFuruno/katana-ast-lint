use katana_ast_lint::Violation;
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[path = "workspace_paths/exemptions.rs"]
mod exemptions;

struct Workspace {
    root: PathBuf,
    member: PathBuf,
}

impl Workspace {
    fn new(root: &Path) -> Self {
        let root = root.canonicalize().unwrap();
        let member = root.join("crates/member");
        fs::create_dir_all(member.join("src")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\"crates/member\"]\nresolver = \"3\"\n",
        )
        .unwrap();
        fs::write(
            member.join("Cargo.toml"),
            "[package]\nname = \"member\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .unwrap();
        fs::write(member.join("src/lib.rs"), "").unwrap();
        Self { root, member }
    }

    fn write_probe(&self, dir: &str) -> PathBuf {
        let dir = self.root.join(dir);
        fs::create_dir_all(&dir).unwrap();
        let file = dir.join("probe.rs");
        let source = format!(
            "#[cfg(test)]\nmod fixture;\n{}fn production() {{\n{}}}\n",
            "\n".repeat(200),
            "    let _ = ();\n".repeat(31)
        );
        fs::write(&file, source).unwrap();
        fs::write(dir.join("fixture.rs"), "").unwrap();
        file
    }

    fn write_config(&self, name: &str, config: Value) -> PathBuf {
        let path = self.root.join(name);
        fs::write(&path, config.to_string()).unwrap();
        path
    }

    fn diagnostics(&self, config_override: Option<&Path>) -> Vec<Violation> {
        let root = run_check(&self.root, config_override);
        let member = run_check(&self.member, config_override);
        assert_eq!(root.status.code(), Some(1));
        let root_diagnostics = String::from_utf8(root.stdout).unwrap();
        let member_diagnostics = String::from_utf8(member.stdout).unwrap();
        assert_eq!(
            root_diagnostics, member_diagnostics,
            "workspace and member diagnostics must match"
        );
        assert_eq!(member.status.code(), Some(1));

        serde_json::Deserializer::from_str(&root_diagnostics)
            .into_iter::<Vec<Violation>>()
            .flat_map(|group| group.expect("valid JSON diagnostics"))
            .collect()
    }
}

fn run_check(dir: &Path, config_override: Option<&Path>) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kal"));
    command.args(["check", "--json"]).current_dir(dir);
    if let Some(path) = config_override {
        command.env("KAL_CONFIG_PATH", path);
    } else {
        command.env_remove("KAL_CONFIG_PATH");
    }
    command.output().unwrap()
}

#[test]
fn workspace_relative_inputs_report_the_same_violations_from_root_and_member() {
    let temp = tempfile::tempdir().unwrap();
    let workspace = Workspace::new(temp.path());
    let file = workspace.write_probe("crates/member/src");
    let function = workspace.write_probe("per-rule");
    workspace.write_config(
        "kal.json",
        json!({
            "source_roots": ["crates/member/src"],
            "rules": {"function-length": {"inputs": ["per-rule"]}},
        }),
    );

    let violations = workspace.diagnostics(None);
    assert_eq!(violations.len(), 2);
    assert_eq!(violations[0].file.canonicalize().unwrap(), file);
    assert!(
        violations[0]
            .message
            .starts_with("File exceeds 200-line limit")
    );
    assert_eq!(violations[1].file.canonicalize().unwrap(), function);
    assert!(
        violations[1]
            .message
            .starts_with("Function `production` exceeds 30-line limit")
    );
}

#[test]
fn explicit_config_takes_precedence_and_preserves_absolute_inputs() {
    let temp = tempfile::tempdir().unwrap();
    let workspace = Workspace::new(temp.path());
    let file = workspace.write_probe("absolute-source");
    let absolute = workspace.write_probe("absolute-rule");
    let relative = workspace.write_probe("relative-rule");
    workspace.write_config("kal.json", json!({"source_roots": ["unused"]}));
    let config = workspace.write_config(
        "crates/member/override.json",
        json!({
            "source_roots": [file.parent().unwrap()],
            "rules": {
                "function-length": {
                    "inputs": [absolute.parent().unwrap(), "relative-rule"],
                }
            },
        }),
    );

    let violations = workspace.diagnostics(Some(&config));
    let files: Vec<_> = violations
        .iter()
        .map(|violation| violation.file.canonicalize().unwrap())
        .collect();
    assert_eq!(files, [file, absolute, relative]);
}

#[test]
fn workspace_parent_named_tests_does_not_hide_production_violations() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("tests/project");
    fs::create_dir_all(&root).unwrap();
    let workspace = Workspace::new(&root);
    let production = workspace.write_probe("crates/member/src");
    workspace.write_probe("crates/member/src/tests");
    workspace.write_config("kal.json", json!({"source_roots": ["crates/member/src"]}));

    let violations = workspace.diagnostics(None);
    assert_eq!(violations.len(), 2);
    assert!(
        violations[0]
            .message
            .starts_with("File exceeds 200-line limit")
    );
    assert!(
        violations[1]
            .message
            .starts_with("Function `production` exceeds 30-line limit")
    );
    for violation in violations {
        assert_eq!(violation.file.canonicalize().unwrap(), production);
    }
}

#[test]
fn workspace_type_separation_preserves_exemptions_and_reports_the_same_path() {
    let temp = tempfile::tempdir().unwrap();
    let workspace = Workspace::new(temp.path());
    let source = format!(
        "pub struct Record;\n{}impl Record {{ fn run(&self) {{}} }}\n",
        "\n".repeat(250)
    );
    for name in ["types", "models", "state", "migration"] {
        let dir = workspace.root.join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("record.rs"), &source).unwrap();
    }
    let production = workspace.member.join("src/record.rs");
    fs::write(&production, source).unwrap();
    workspace.write_config(
        "kal.json",
        json!({"rules": {"file-length": {"enabled": false}}}),
    );

    let violations = workspace.diagnostics(None);
    assert_eq!(violations.len(), 1);
    assert_eq!(violations[0].file.canonicalize().unwrap(), production);
    assert!(violations[0].message.starts_with("Mixed logic and data."));
}
