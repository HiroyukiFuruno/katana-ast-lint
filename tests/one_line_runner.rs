use katana_ast_lint::KatanaAstLint;
use katana_ast_lint::config::KalConfig;
use std::path::PathBuf;

struct TempProject {
    dir: tempfile::TempDir,
}

impl TempProject {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir must be created");
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).expect("src directory must be created");
        std::fs::write(src.join("lib.rs"), "fn f() { todo!() }\n")
            .expect("source file must be written");
        Self { dir }
    }

    fn path(&self) -> PathBuf {
        self.dir.path().to_path_buf()
    }
}

#[test]
fn with_config_lints_current_directory_when_roots_are_not_configured() {
    let project = TempProject::new();
    let original_dir = std::env::current_dir().unwrap();

    // We can't easily test from_workspace() because it uses current_dir() which is global to the process
    // and tests run in parallel.
    // However, we can test with_config() which now also defaults to current_dir().

    std::env::set_current_dir(project.path()).unwrap();

    let result = std::panic::catch_unwind(|| {
        KatanaAstLint::with_config(KalConfig::default()).assert_clean();
    });

    std::env::set_current_dir(&original_dir).unwrap();

    assert!(
        result.is_err(),
        "Linter should have found todo!() in lib.rs"
    );
}

#[test]
fn with_config_respects_custom_config() {
    let mut config = KalConfig::default();
    config.rules.insert(
        "lazy-code".to_string(),
        katana_ast_lint::config::RuleConfig {
            enabled: Some(false),
            ..Default::default()
        },
    );

    let project = TempProject::new();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(project.path()).unwrap();

    let result = std::panic::catch_unwind(|| {
        KatanaAstLint::with_config(config).assert_clean();
    });

    std::env::set_current_dir(original_dir).unwrap();
    result.unwrap();
}
