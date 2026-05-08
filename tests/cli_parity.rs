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

#[test]
fn cli_check_clean_project_exits_0() {
    let dir = tempdir().expect("failed to create temp dir");
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "fn main() {}").unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"
[package]
name = "test-project"
version = "0.1.0"
edition = "2024"

[workspace]
"#,
    )
    .unwrap();

    let output = Command::new(get_kal_path())
        .args(["check"])
        .current_dir(dir.path())
        .output()
        .expect("failed to execute process");

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
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "fn main() { todo!() }").unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"
[package]
name = "test-project-violation"
version = "0.1.0"
edition = "2024"

[workspace]
"#,
    )
    .unwrap();

    let output = Command::new(get_kal_path())
        .args(["check"])
        .current_dir(dir.path())
        .output()
        .expect("failed to execute process");

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

    let output = Command::new(get_kal_path())
        .args(["check"])
        .current_dir(dir.path())
        .output()
        .expect("failed to execute process");

    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("Invalid KAL configuration"));
}

#[test]
fn cli_and_api_text_parity() {
    let _guard = CWD_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let dir = tempdir().expect("failed to create temp dir");
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "fn main() { todo!() }").unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"
[package]
name = "test-project-parity"
version = "0.1.0"
edition = "2024"

[workspace]
"#,
    )
    .unwrap();

    // Run via CLI
    let cli_output = Command::new(get_kal_path())
        .args(["check"])
        .current_dir(dir.path())
        .output()
        .expect("failed to execute process");
    let cli_stdout = String::from_utf8_lossy(&cli_output.stdout);

    // Run via API
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();

    let linter = katana_ast_lint::KatanaAstLint::from_workspace();
    let violations = linter.violations();

    std::env::set_current_dir(original_dir).unwrap();

    // Check that CLI contains same key information
    assert!(cli_stdout.contains("lazy-code"));
    assert!(cli_stdout.contains("todo!()"));
    assert!(violations.iter().any(|v| v.message.contains("todo!()")));
}

#[test]
fn cli_and_api_json_parity() {
    let _guard = CWD_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

    let dir = tempdir().expect("failed to create temp dir");
    let src = dir.path().join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("lib.rs"), "fn main() { todo!() }").unwrap();
    fs::write(
        dir.path().join("kal.json"),
        r#"{"reporter": {"mode": "json"}}"#,
    )
    .unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        r#"
[package]
name = "test-project-json-parity"
version = "0.1.0"
edition = "2024"

[workspace]
"#,
    )
    .unwrap();

    // Run via CLI
    let cli_output = Command::new(get_kal_path())
        .args(["check"])
        .current_dir(dir.path())
        .output()
        .expect("failed to execute process");
    let cli_stdout = String::from_utf8_lossy(&cli_output.stdout);

    let cli_violations: Vec<katana_ast_lint::Violation> =
        serde_json::from_str(&cli_stdout).expect("CLI output should be valid JSON");

    // Run via API
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let linter = katana_ast_lint::KatanaAstLint::from_workspace();
    let api_violations = linter.violations();
    std::env::set_current_dir(original_dir).unwrap();

    assert_eq!(cli_violations.len(), api_violations.len());
    assert_eq!(cli_violations[0].message, api_violations[0].message);
    assert_eq!(
        cli_violations[0].file.file_name(),
        api_violations[0].file.file_name()
    );
}
