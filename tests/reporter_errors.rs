#[cfg(unix)]
mod unix {
    use katana_ast_lint::config::{ReporterConfig, ReporterMode};
    use katana_ast_lint::utils::ViolationReporterOps;
    use katana_ast_lint::{KalRunError, Violation};
    use std::ffi::OsString;
    #[cfg(target_os = "linux")]
    use std::fs;
    use std::os::unix::ffi::OsStringExt;
    use std::path::PathBuf;
    #[cfg(target_os = "linux")]
    use std::process::Command;

    #[cfg(target_os = "linux")]
    fn get_kal_path() -> String {
        let mut path = std::env::current_exe().expect("current executable path must be available");
        path.pop();
        path.pop();
        path.join("kal")
            .to_str()
            .expect("kal path must be UTF-8")
            .to_string()
    }

    #[test]
    fn json_reporter_returns_system_error_for_non_utf8_paths() {
        let violation = Violation::err(
            PathBuf::from(OsString::from_vec(b"invalid-\xFF.rs".to_vec())),
            1,
            1,
            "invalid path should fail JSON serialization",
        );
        let config = ReporterConfig {
            mode: Some(ReporterMode::Json),
        };

        let error =
            ViolationReporterOps::try_report("test-rule", "fix source", &[violation], &config)
                .expect_err("non-UTF-8 paths must fail as system errors");

        match error {
            KalRunError::System(message) => {
                assert!(message.contains("Failed to serialize AST lint violations as JSON"));
            }
            other => panic!("expected system error, got {other:?}"),
        }
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn cli_json_reporter_serialization_failure_exits_2() {
        let dir = tempfile::tempdir().expect("tempdir must be created");
        let src = dir.path().join("src");
        fs::create_dir_all(&src).expect("src directory must be created");
        fs::write(
            dir.path().join("Cargo.toml"),
            r#"
[package]
name = "test-project-non-utf8-path"
version = "0.1.0"
edition = "2024"

[workspace]
"#,
        )
        .expect("Cargo.toml must be written");
        fs::write(
            src.join(PathBuf::from(OsString::from_vec(
                b"invalid-\xFF.rs".to_vec(),
            ))),
            "fn f() { todo!() }\n",
        )
        .expect("source file must be written");

        let output = Command::new(get_kal_path())
            .args(["check", "--json"])
            .current_dir(dir.path())
            .output()
            .expect("kal check must execute");

        assert_eq!(output.status.code(), Some(2));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(stderr.contains("Failed to serialize AST lint violations as JSON"));
    }
}
