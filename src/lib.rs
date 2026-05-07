#![deny(warnings, clippy::all)]
#![allow(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::too_many_lines,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_or_default,
    clippy::wildcard_imports,
    clippy::match_wild_err_arm,
    clippy::let_and_return,
    clippy::manual_ok_err,
    clippy::cognitive_complexity
)]

pub mod config;
pub mod rules;
pub mod utils;

use crate::config::Severity;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};

/* WHY: Domain entities for linter violation reporting and JSON AST traversal. */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Violation {
    pub file: PathBuf,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub severity: Severity,
}

impl Violation {
    pub fn err(file: PathBuf, line: usize, column: usize, message: impl Into<String>) -> Self {
        Self {
            file,
            line,
            column,
            message: message.into(),
            severity: Severity::Error,
        }
    }

    pub fn warn(file: PathBuf, line: usize, column: usize, message: impl Into<String>) -> Self {
        Self {
            file,
            line,
            column,
            message: message.into(),
            severity: Severity::Warning,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsonNodeKind {
    Object,
    Array,
    String,
    Number,
    Bool,
    Null,
}

impl JsonNodeKind {
    pub fn from_value(value: &Value) -> Self {
        match value {
            Value::Object(_) => Self::Object,
            Value::Array(_) => Self::Array,
            Value::String(_) => Self::String,
            Value::Number(_) => Self::Number,
            Value::Bool(_) => Self::Bool,
            Value::Null => Self::Null,
        }
    }
}

impl std::fmt::Display for JsonNodeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Object => write!(f, "object"),
            Self::Array => write!(f, "array"),
            Self::String => write!(f, "string"),
            Self::Number => write!(f, "number"),
            Self::Bool => write!(f, "bool"),
            Self::Null => write!(f, "null"),
        }
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/* WHY: Cache parsed ASTs to avoid redundant parsing across multiple linter tests.
We use thread_local to avoid requiring Sync on the syn::File type while still
achieving significant speedups since tests run in long-lived threads. */
thread_local! {
    static AST_CACHE: RefCell<HashMap<PathBuf, Rc<syn::File>>> = RefCell::new(HashMap::new());
}

pub struct AstLinterOps;

impl AstLinterOps {
    fn find_config_file(target_dirs: &[PathBuf]) -> Option<PathBuf> {
        if let Ok(env_path) = std::env::var("KAL_CONFIG_PATH") {
            let path = PathBuf::from(env_path);
            return path.exists().then_some(path);
        }

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

    fn load_config(target_dirs: &[PathBuf]) -> config::KalConfig {
        match Self::find_config_file(target_dirs) {
            Some(config_path) => {
                config::KalConfig::load_from_path(&config_path).unwrap_or_else(|e| {
                    panic!(
                        "Invalid KAL configuration at {}: {}",
                        config_path.display(),
                        e
                    )
                })
            }
            None => config::KalConfig::load_default(),
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
        let rule_config = config
            .rules
            .get(rule_name)
            .cloned()
            .unwrap_or_else(|| config::RuleConfig::default_for_rule(rule_name));

        if !rule_config.enabled.unwrap_or(true) {
            return;
        }

        let mut all_violations: Vec<Violation> = Vec::new();

        let source_roots = if !rule_config.inputs.is_empty() {
            &rule_config.inputs
        } else if !config.source_roots.is_empty() {
            &config.source_roots
        } else {
            target_dirs
        };

        for target_dir in source_roots {
            Self::lint_directory(target_dir, &rule_config, lint_fn, &mut all_violations);
        }

        utils::ViolationReporterOps::report(rule_name, hint, &all_violations, &config.reporter);
    }

    fn lint_directory(
        target_dir: &Path,
        rule_config: &config::RuleConfig,
        lint_fn: impl Fn(&Path, &syn::File, &config::RuleConfig) -> Vec<Violation>,
        all_violations: &mut Vec<Violation>,
    ) {
        let now = chrono::Utc::now().naive_utc().date();

        for file in &utils::LinterFileOps::collect_rs_files(target_dir) {
            if rule_config.allow.iter().any(|a| {
                if !file.ends_with(&a.path) {
                    return false;
                }
                if let Some(expiry) = &a.expires_at {
                    match chrono::NaiveDate::parse_from_str(expiry, "%Y-%m-%d") {
                        Ok(expiry_date) => return now < expiry_date,
                        Err(e) => {
                            eprintln!("Warning: Failed to parse expiry date '{}': {}", expiry, e);
                            return false;
                        }
                    }
                }
                true
            }) {
                continue;
            }

            let syntax = match Self::get_cached_ast(file) {
                Ok(s) => s,
                Err(errors) => {
                    all_violations.extend(errors);
                    continue;
                }
            };

            let mut violations = lint_fn(file, &syntax, rule_config);
            let severity = rule_config.severity.unwrap_or(Severity::Error);
            for v in &mut violations {
                v.severity = severity;
            }
            all_violations.extend(violations);
        }
    }

    fn get_cached_ast(file: &Path) -> Result<Rc<syn::File>, Vec<Violation>> {
        AST_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if let Some(cached) = cache.get(file) {
                return Ok(Rc::clone(cached));
            }

            match utils::LinterParserOps::parse_file(file) {
                Ok(syntax) => {
                    let syntax_rc = Rc::new(syntax);
                    cache.insert(file.to_path_buf(), Rc::clone(&syntax_rc));
                    Ok(syntax_rc)
                }
                Err(errors) => Err(errors),
            }
        })
    }
}
