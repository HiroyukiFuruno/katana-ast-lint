use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub struct LinterFileOps;

impl LinterFileOps {
    pub fn collect_rs_files(root: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let walker = WalkBuilder::new(root)
            .standard_filters(true)
            .require_git(false)
            .build();

        for entry in walker.flatten() {
            let path = entry.path();
            if path.is_file()
                && path.extension().is_some_and(|ext| ext == "rs")
                && !path.components().any(|c| c.as_os_str() == "tests")
            {
                files.push(path.to_path_buf());
            }
        }

        files.sort();
        files
    }

    pub fn collect_files_by_extension(root: &Path, extension: &str) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let walker = WalkBuilder::new(root)
            .standard_filters(true)
            .require_git(false)
            .build();

        for entry in walker.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == extension) {
                files.push(path.to_path_buf());
            }
        }
        files.sort();
        files
    }

    pub fn workspace_root() -> Result<&'static Path, String> {
        use std::sync::OnceLock;
        static ROOT: OnceLock<Option<PathBuf>> = OnceLock::new();
        let root = ROOT.get_or_init(|| {
            std::env::var_os("KATANA_AST_LINT_TARGET_DIR")
                .map(PathBuf::from)
                .or_else(|| std::env::current_dir().ok())
        });

        match root.as_ref() {
            Some(path) => Ok(path.as_path()),
            None => Err("Workspace root not found".to_string()),
        }
    }
}
