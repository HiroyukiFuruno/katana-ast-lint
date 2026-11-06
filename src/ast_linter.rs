use crate::{KalRunError, Violation, config, rust_sources::RustSources, utils};
use std::path::{Path, PathBuf};

pub struct AstLinterOps;

impl AstLinterOps {
    fn find_config_file(target_dirs: &[PathBuf]) -> Option<PathBuf> {
        match std::env::var("KAL_CONFIG_PATH") {
            Ok(env_path) => {
                let path = PathBuf::from(env_path);
                path.exists().then_some(path)
            }
            Err(_) => Self::find_implicit_config(target_dirs),
        }
    }

    fn find_implicit_config(target_dirs: &[PathBuf]) -> Option<PathBuf> {
        for target_dir in target_dirs {
            if let Some(config_path) = Self::find_config_from(target_dir) {
                return Some(config_path);
            }
        }

        Self::find_config_from(&std::env::current_dir().ok()?)
    }

    fn find_config_from(start: &Path) -> Option<PathBuf> {
        let mut curr = if start.is_file() {
            start.parent()?.to_path_buf()
        } else {
            start.to_path_buf()
        };

        loop {
            let config = curr.join("kal.json");
            if config.exists() {
                return Some(config);
            }
            if !curr.pop() {
                break;
            }
        }
        None
    }

    pub(crate) fn load_config(target_dirs: &[PathBuf]) -> config::KalConfig {
        Self::try_load_config(target_dirs).unwrap_or_else(|e| panic!("{}", e))
    }

    pub(crate) fn try_load_config(
        target_dirs: &[PathBuf],
    ) -> Result<config::KalConfig, KalRunError> {
        match Self::find_config_file(target_dirs) {
            Some(config_path) => config::KalConfig::load_from_path(&config_path).map_err(|e| {
                KalRunError::Configuration(format!(
                    "Invalid KAL configuration at {}: {}",
                    config_path.display(),
                    e
                ))
            }),
            None => Ok(config::KalConfig::load_default()),
        }
    }

    pub fn run(
        rule_name: &str,
        hint: &str,
        target_dirs: &[PathBuf],
        lint_fn: fn(&Path, &syn::File) -> Vec<Violation>,
    ) {
        let config = Self::load_config(target_dirs);
        Self::run_inner(rule_name, hint, target_dirs, &config, |path, syntax, _| {
            lint_fn(path, syntax)
        });
    }

    pub fn run_with_configured_rule(
        rule_name: &str,
        hint: &str,
        target_dirs: &[PathBuf],
        lint_fn: fn(&Path, &syn::File, &config::RuleConfig) -> Vec<Violation>,
    ) {
        let config = Self::load_config(target_dirs);
        Self::run_inner(rule_name, hint, target_dirs, &config, lint_fn);
    }

    pub fn run_with_config(
        rule_name: &str,
        hint: &str,
        target_dirs: &[PathBuf],
        config: &config::KalConfig,
        lint_fn: fn(&Path, &syn::File, &config::RuleConfig) -> Vec<Violation>,
    ) {
        Self::run_inner(rule_name, hint, target_dirs, config, lint_fn);
    }

    fn run_inner(
        rule_name: &str,
        hint: &str,
        target_dirs: &[PathBuf],
        config: &config::KalConfig,
        lint_fn: impl Fn(&Path, &syn::File, &config::RuleConfig) -> Vec<Violation> + Copy,
    ) {
        let rule_config = config.rule_config(rule_name);
        if !rule_config.enabled.unwrap_or(true) {
            return;
        }

        let mut sources = RustSources::default();
        let mut all_violations = Vec::new();
        for root in config.rule_roots(&rule_config, target_dirs) {
            sources.lint_directory(root, &rule_config, lint_fn, &mut all_violations);
        }

        utils::ViolationReporterOps::report(rule_name, hint, &all_violations, &config.reporter);
    }
}
