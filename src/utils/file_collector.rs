use ignore::WalkBuilder;
use std::path::{Path, PathBuf};

pub struct LinterFileOps;

impl LinterFileOps {
    pub fn collect_rs_files(root: &Path) -> Vec<PathBuf> {
        Self::collect_rs_files_in_scope(root, None)
    }

    pub(crate) fn collect_rs_files_in_scope(root: &Path, scope: Option<&Path>) -> Vec<PathBuf> {
        Self::collect_files(root, "rs", |path| {
            !Self::path_in_scope(path, scope)
                .components()
                .any(|component| component.as_os_str() == "tests")
        })
    }

    pub(crate) fn path_in_scope<'a>(path: &'a Path, scope: Option<&Path>) -> &'a Path {
        scope
            .and_then(|root| path.strip_prefix(root).ok())
            .unwrap_or(path)
    }

    pub fn collect_files_by_extension(root: &Path, extension: &str) -> Vec<PathBuf> {
        Self::collect_files(root, extension, |_| true)
    }

    fn collect_files(
        root: &Path,
        extension: &str,
        include: impl Fn(&Path) -> bool,
    ) -> Vec<PathBuf> {
        let walker = WalkBuilder::new(root)
            .standard_filters(true)
            .require_git(false)
            .build();

        let mut files = Vec::new();
        for entry in walker.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == extension)
                && path.is_file()
                && include(path)
            {
                files.push(entry.into_path());
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
