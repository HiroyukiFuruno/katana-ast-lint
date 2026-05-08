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
use crate::rules::{RULE_CATALOG, RuleImplementation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};

/* WHY: Domain entities for linter violation reporting and JSON AST traversal. */
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

pub struct KatanaAstLint {
    config: config::KalConfig,
    target_dirs: Vec<PathBuf>,
}

impl KatanaAstLint {
    pub fn from_workspace() -> Self {
        Self::try_from_workspace().unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn try_from_workspace() -> Result<Self, String> {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let root = Self::find_workspace_root(&current_dir).unwrap_or_else(|| current_dir.clone());
        let config = AstLinterOps::try_load_config(std::slice::from_ref(&root))?;
        Ok(Self {
            config,
            target_dirs: vec![root],
        })
    }

    fn find_workspace_root(start: &Path) -> Option<PathBuf> {
        let mut curr = start.to_path_buf();
        loop {
            let manifest = curr.join("Cargo.toml");
            if manifest.exists()
                && std::fs::read_to_string(&manifest)
                    .ok()
                    .is_some_and(|s| s.contains("[workspace]"))
            {
                return Some(curr);
            }
            if !curr.pop() {
                break;
            }
        }
        None
    }

    pub fn with_config(config: config::KalConfig) -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            config,
            target_dirs: vec![current_dir],
        }
    }

    pub fn violations(&self) -> Vec<Violation> {
        let mut results = self.lint_all();

        let mut sorted_rule_ids: Vec<_> = results.keys().copied().collect();
        sorted_rule_ids.sort();

        let mut all = Vec::new();
        for rule_id in sorted_rule_ids {
            if let Some(violations) = results.remove(rule_id) {
                all.extend(violations);
            }
        }
        all
    }

    pub fn assert_clean(&self) {
        self.try_assert_clean().unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn try_assert_clean(&self) -> Result<(), String> {
        let results = self.lint_all();
        if results.is_empty() {
            return Ok(());
        }

        let mut errors = Vec::new();
        let sorted_keys: Vec<_> = {
            let mut keys: Vec<_> = results.keys().collect();
            keys.sort();
            keys
        };

        for rule_id in sorted_keys {
            let rule_violations = results.get(rule_id).unwrap();
            let hint = RULE_CATALOG
                .iter()
                .find(|r| r.id == *rule_id)
                .map(|r| r.hint)
                .unwrap_or("Check the documentation for remediation guidance.");

            if let Err(e) = utils::ViolationReporterOps::try_report(
                rule_id,
                hint,
                rule_violations,
                &self.config.reporter,
            ) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("\n"))
        }
    }

    fn lint_all(&self) -> HashMap<&'static str, Vec<Violation>> {
        let mut results = HashMap::new();

        for rule_def in RULE_CATALOG.iter() {
            let mut rule_config = self
                .config
                .rules
                .get(rule_def.id)
                .cloned()
                .unwrap_or_else(|| config::RuleConfig::default_for_rule(rule_def.id));

            if !rule_config.enabled.unwrap_or(true) {
                continue;
            }

            if rule_config.threshold.is_none() {
                rule_config.threshold = rule_def.default_threshold;
            }

            let mut rule_violations = Vec::new();

            let source_roots = if !rule_config.inputs.is_empty() {
                &rule_config.inputs
            } else if !self.config.source_roots.is_empty() {
                &self.config.source_roots
            } else {
                &self.target_dirs
            };

            for source_root in source_roots {
                match &rule_def.implementation {
                    RuleImplementation::RustFile(lint_fn) => {
                        AstLinterOps::lint_directory(
                            source_root,
                            &rule_config,
                            *lint_fn,
                            &mut rule_violations,
                        );
                    }
                    RuleImplementation::Directory(lint_dir_fn) => {
                        let violations = lint_dir_fn(source_root);
                        let severity = rule_config.severity.unwrap_or(rule_def.default_severity);
                        for mut v in violations {
                            v.severity = severity;
                            rule_violations.push(v);
                        }
                    }
                }
            }

            if !rule_violations.is_empty() {
                results.insert(rule_def.id, rule_violations);
            }
        }

        results
    }
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

    pub(crate) fn load_config(target_dirs: &[PathBuf]) -> config::KalConfig {
        Self::try_load_config(target_dirs).unwrap_or_else(|e| panic!("{}", e))
    }

    pub(crate) fn try_load_config(target_dirs: &[PathBuf]) -> Result<config::KalConfig, String> {
        match Self::find_config_file(target_dirs) {
            Some(config_path) => config::KalConfig::load_from_path(&config_path).map_err(|e| {
                format!(
                    "Invalid KAL configuration at {}: {}",
                    config_path.display(),
                    e
                )
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

    pub(crate) fn lint_directory(
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
