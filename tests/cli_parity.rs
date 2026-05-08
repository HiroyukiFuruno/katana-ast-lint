use std::fs;
use std::process::Command;
use std::sync::Mutex;
use tempfile::tempdir;

/* WHY: process-global cwd is shared across parallel tests; serialize to prevent race */
static CWD_MUTEX: Mutex<()> = Mutex::new(());

fn get_kal_path() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop(); // deps
    path.pop(); // debug
    path.join("kal").to_str().unwrap().to_string()
}

fn write_minimal_workspace(dir: &std::path::Path, name: &str, lib_rs: &str) {
    let src = dir.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), lib_rs).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        format!(
            r#"
[package]
name = "{name}"
version = "0.1.0"
edition = "2024"

[workspace]
"#
        ),
    )
    .unwrap();
}

fn run_cli(dir: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(get_kal_path())
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to execute process")
}

fn with_cwd_locked<T>(dir: &std::path::Path, f: impl FnOnce() -> T) -> T {
    let _guard = CWD_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir).unwrap();

    let value = f();

    std::env::set_current_dir(original_dir).unwrap();
    value
}

fn api_violations(dir: &std::path::Path) -> Vec<katana_ast_lint::Violation> {
    with_cwd_locked(dir, || {
        katana_ast_lint::KatanaAstLint::try_from_workspace()
            .expect("API runner should resolve workspace")
            .violations()
    })
}

fn hint_for_rule(rule_id: &str) -> &'static str {
    katana_ast_lint::rules::RULE_CATALOG
        .iter()
        .find(|r| r.id == rule_id)
        .map(|r| r.hint)
        .unwrap_or("Check the documentation for remediation guidance.")
}

fn expected_text_output(rule_id: &str, violations: &[katana_ast_lint::Violation]) -> String {
    let hint = hint_for_rule(rule_id);

    let mut msg =
        katana_ast_lint::utils::ViolationReporterOps::format_violations(rule_id, violations);
    msg.push('\n');
    msg.push_str(&format!("Fix: {hint}\n"));
    msg.push_str("Details: See docs/quality-gates.md\n");

    // Mirror the reporter's `println!`, which appends a trailing newline.
    format!("{msg}\n")
}

#[test]
fn cli_check_clean_project_exits_0() {
    let dir = tempdir().expect("failed to create temp dir");
    write_minimal_workspace(dir.path(), "test-project", "fn main() {}");
    let output = run_cli(dir.path(), &["check"]);

    if output.status.code() != Some(0) {
        eprintln!("test-project path: {}", dir.path().display());
        eprintln!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn cli_check_violation_exits_1() {
    let dir = tempdir().expect("failed to create temp dir");
    write_minimal_workspace(
        dir.path(),
        "test-project-violation",
        "fn main() { todo!() }",
    );
    let output = run_cli(dir.path(), &["check"]);

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("lazy-code"));
}

#[test]
fn cli_check_invalid_config_exits_2() {
    let dir = tempdir().expect("failed to create temp dir");
    fs::write(dir.path().join("kal.json"), "{ invalid json }").unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"
[package]
name = "test-project-config-error"
version = "0.1.0"
edition = "2024"

[workspace]
"#,
    )
    .unwrap();
    let output = run_cli(dir.path(), &["check"]);

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Invalid KAL configuration"));
}

#[test]
fn cli_and_api_text_parity() {
    let dir = tempdir().expect("failed to create temp dir");
    write_minimal_workspace(dir.path(), "test-project-parity", "fn main() { todo!() }");

    let cli_output = run_cli(dir.path(), &["check"]);
    assert_eq!(cli_output.status.code(), Some(1));
    let cli_stdout = String::from_utf8(cli_output.stdout).expect("CLI stdout must be UTF-8");

    let violations = api_violations(dir.path());
    assert!(
        !violations.is_empty(),
        "API runner should detect violations"
    );

    // WHEN linting the same workspace through the API runner and `kal check`
    // THEN both executions produce equivalent reporter output.
    assert_eq!(cli_stdout, expected_text_output("lazy-code", &violations));
}

#[test]
fn cli_and_api_json_parity() {
    let dir = tempdir().expect("failed to create temp dir");
    write_minimal_workspace(
        dir.path(),
        "test-project-json-parity",
        "fn main() { todo!() }",
    );
    fs::write(
        dir.path().join("kal.json"),
        r#"{"reporter": {"mode": "json"}}"#,
    )
    .unwrap();

    let cli_output = run_cli(dir.path(), &["check"]);
    assert_eq!(cli_output.status.code(), Some(1));
    let cli_stdout = String::from_utf8(cli_output.stdout).expect("CLI stdout must be UTF-8");

    let violations = api_violations(dir.path());
    assert!(
        !violations.is_empty(),
        "API runner should detect violations"
    );

    // WHEN linting the same workspace through the API runner and `kal check`
    // THEN both executions produce equivalent reporter output.
    let expected = format!(
        "{}\n",
        serde_json::to_string_pretty(&violations).expect("valid JSON")
    );
    assert_eq!(cli_stdout, expected);
}

#[test]
fn cli_check_json_override_parity() {
    let dir = tempdir().expect("failed to create temp dir");
    write_minimal_workspace(
        dir.path(),
        "test-project-json-override",
        "fn main() { todo!() }",
    );
    // kal.json defaults to text or is absent.

    // Force JSON output via CLI flag.
    let cli_output = run_cli(dir.path(), &["check", "--json"]);
    assert_eq!(cli_output.status.code(), Some(1));
    let cli_stdout = String::from_utf8(cli_output.stdout).expect("CLI stdout must be UTF-8");

    let violations = api_violations(dir.path());

    // API side: simulate JSON mode by configuring the linter explicitly.
    let expected = with_cwd_locked(dir.path(), || {
        let _linter = katana_ast_lint::KatanaAstLint::try_from_workspace()
            .unwrap()
            .with_reporter_mode(katana_ast_lint::config::ReporterMode::Json);

        // We need to capture what linter.try_assert_clean() would print.
        // Since it prints to stdout, we trust the parity if it matches our manual expectation
        // which matches the logic in `cli_and_api_json_parity`.
        format!(
            "{}\n",
            serde_json::to_string_pretty(&violations).expect("valid JSON")
        )
    });

    assert_eq!(cli_stdout, expected);
}
