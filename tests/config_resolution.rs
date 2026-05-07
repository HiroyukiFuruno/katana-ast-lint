use katana_ast_lint::AstLinterOps;
use katana_ast_lint::rules::LazyCodeOps;
use std::path::PathBuf;

struct TempProject {
    dir: tempfile::TempDir,
}

impl TempProject {
    fn new() -> Self {
        let dir = tempfile::tempdir().expect("tempdir must be created");
        let src = dir.path().join("src");
        std::fs::create_dir_all(&src).expect("src directory must be created");
        std::fs::write(src.join("lib.rs"), "pub fn f() { todo!() }\n")
            .expect("source file must be written");
        Self { dir }
    }

    fn source_roots(&self) -> Vec<PathBuf> {
        vec![self.dir.path().join("src")]
    }

    fn write_config(&self, enabled: bool) {
        std::fs::write(
            self.dir.path().join("kal.json"),
            format!(
                r#"{{
  "rules": {{
    "lazy-code": {{
      "enabled": {enabled}
    }}
  }}
}}"#
            ),
        )
        .expect("kal.json must be written");
    }
}

#[test]
fn run_resolves_config_from_lint_targets() {
    let project = TempProject::new();
    project.write_config(false);

    AstLinterOps::run(
        "lazy-code",
        "Remove lazy macros.",
        &project.source_roots(),
        LazyCodeOps::lint,
    );
}

#[test]
fn run_reloads_config_after_kal_json_changes() {
    let project = TempProject::new();
    project.write_config(false);

    AstLinterOps::run(
        "lazy-code",
        "Remove lazy macros.",
        &project.source_roots(),
        LazyCodeOps::lint,
    );

    project.write_config(true);
    let result = std::panic::catch_unwind(|| {
        AstLinterOps::run(
            "lazy-code",
            "Remove lazy macros.",
            &project.source_roots(),
            LazyCodeOps::lint,
        );
    });

    assert!(result.is_err());
}
