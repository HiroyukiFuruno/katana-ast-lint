use crate::rules::{RULE_CATALOG, RuleDefinition, RuleImplementation};
use crate::{AstLinterOps, KalRunError, Violation, config, rust_sources::RustSources, utils};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub struct KatanaAstLint {
    config: config::KalConfig,
    root: PathBuf,
    workspace_inputs: bool,
}

impl KatanaAstLint {
    pub fn from_workspace() -> Self {
        Self::try_from_workspace().unwrap_or_else(|e| panic!("{}", e))
    }

    /// Loads workspace configuration and resolves relative input paths from the workspace root.
    pub fn try_from_workspace() -> Result<Self, KalRunError> {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let root = Self::find_workspace_root(&current_dir).unwrap_or_else(|| current_dir.clone());
        let mut config = AstLinterOps::try_load_config(std::slice::from_ref(&root))?;
        config.resolve_input_roots(&root);
        Ok(Self {
            config,
            root,
            workspace_inputs: true,
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
            root: current_dir,
            workspace_inputs: false,
        }
    }

    pub fn with_reporter_mode(mut self, mode: config::ReporterMode) -> Self {
        self.config.reporter.mode = Some(mode);
        self
    }

    pub fn violations(&self) -> Vec<Violation> {
        self.lint_all().into_values().flatten().collect()
    }

    pub fn assert_clean(&self) {
        self.try_assert_clean().unwrap_or_else(|e| panic!("{}", e))
    }

    pub fn try_assert_clean(&self) -> Result<(), KalRunError> {
        let mut errors = Vec::new();
        for (rule_id, violations) in self.lint_all() {
            let hint = RULE_CATALOG
                .iter()
                .find(|rule| rule.id == rule_id)
                .map(|rule| rule.hint)
                .unwrap_or("Check the documentation for remediation guidance.");

            if let Err(error) = utils::ViolationReporterOps::try_report(
                rule_id,
                hint,
                &violations,
                &self.config.reporter,
            ) {
                errors.push(error);
            }
        }
        Self::combine_report_errors(errors)
    }

    fn combine_report_errors(errors: Vec<KalRunError>) -> Result<(), KalRunError> {
        let mut messages = Vec::with_capacity(errors.len());
        let (mut is_violation, mut is_config) = (false, false);

        for error in errors {
            match error {
                KalRunError::Violations(message) => {
                    messages.push(message);
                    is_violation = true;
                }
                KalRunError::Configuration(message) => {
                    messages.push(message);
                    is_config = true;
                }
                KalRunError::System(message) => messages.push(message),
            }
        }

        if messages.is_empty() {
            return Ok(());
        }
        let combined = messages.join("\n");
        if is_config {
            Err(KalRunError::Configuration(combined))
        } else if is_violation {
            Err(KalRunError::Violations(combined))
        } else {
            Err(KalRunError::System(combined))
        }
    }

    fn lint_all(&self) -> BTreeMap<&'static str, Vec<Violation>> {
        let scope = self.workspace_inputs.then_some(self.root.as_path());
        let mut sources = RustSources::new(scope);
        let mut results = BTreeMap::new();

        for rule in RULE_CATALOG.iter() {
            let mut config = self.config.rule_config(rule.id);
            if !config.enabled.unwrap_or(true) {
                continue;
            }
            config.threshold = config.threshold.or(rule.default_threshold);

            let roots = self
                .config
                .rule_roots(&config, std::slice::from_ref(&self.root));
            let violations = Self::lint_rule(rule, &config, roots, &mut sources);
            if !violations.is_empty() {
                results.insert(rule.id, violations);
            }
        }
        results
    }

    fn lint_rule(
        rule: &RuleDefinition,
        config: &config::RuleConfig,
        roots: &[PathBuf],
        sources: &mut RustSources<'_>,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();
        for root in roots {
            match rule.implementation {
                RuleImplementation::RustFile(lint) => {
                    sources.lint_directory(
                        root,
                        config,
                        |path, scoped_path, syntax, config| {
                            rule.lint_rust_file(lint, path, scoped_path, syntax, config)
                        },
                        &mut violations,
                    );
                }
                RuleImplementation::Directory(lint) => {
                    let severity = config.severity.unwrap_or(rule.default_severity);
                    violations.extend(lint(root).into_iter().map(|mut violation| {
                        violation.severity = severity;
                        violation
                    }));
                }
            }
        }
        violations
    }
}
