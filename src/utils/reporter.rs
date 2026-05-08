use crate::Violation;
use crate::config::{ReporterConfig, ReporterMode, Severity};
use std::path::Path;

pub struct ViolationReporterOps;

impl ViolationReporterOps {
    pub fn format_violations(rule_name: &str, violations: &[Violation]) -> String {
        let mut msg = format!("\n[AST Linter] Rule: {}\n", rule_name);
        for v in violations {
            let sev_label = match v.severity {
                Severity::Error => "Error",
                Severity::Warning => "Warning",
                Severity::Info => "Info",
            };
            msg.push_str(&format!(
                "  [{}] {}:{}:{} — {}\n",
                sev_label,
                v.file.display(),
                v.line,
                v.column,
                v.message
            ));
        }
        msg
    }

    pub fn panic(rule_name: &str, hint: &str, violations: &[Violation]) {
        Self::report(rule_name, hint, violations, &ReporterConfig::default());
    }

    pub fn report(
        rule_name: &str,
        hint: &str,
        violations: &[Violation],
        reporter_config: &ReporterConfig,
    ) {
        Self::try_report(rule_name, hint, violations, reporter_config)
            .unwrap_or_else(|e| panic!("{}", e));
    }

    pub fn try_report(
        rule_name: &str,
        hint: &str,
        violations: &[Violation],
        reporter_config: &ReporterConfig,
    ) -> Result<(), crate::KalRunError> {
        if violations.is_empty() {
            return Ok(());
        }

        match reporter_config.mode.unwrap_or_default() {
            ReporterMode::Text => {
                let mut msg = Self::format_violations(rule_name, violations);
                msg.push('\n');
                msg.push_str(&format!("Fix: {}\n", hint));
                msg.push_str("Details: See docs/quality-gates.md\n");

                println!("{}", msg);

                if violations.iter().any(|v| v.severity == Severity::Error) {
                    return Err(crate::KalRunError::Violations(msg));
                }
            }
            ReporterMode::Json => {
                let json = serde_json::to_string_pretty(violations).map_err(|e| {
                    crate::KalRunError::System(format!(
                        "Failed to serialize AST lint violations as JSON: {e}"
                    ))
                })?;
                println!("{json}");
                if violations.iter().any(|v| v.severity == Severity::Error) {
                    return Err(crate::KalRunError::Violations(
                        "AST Lint failed with errors (JSON output above)".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn locale_violation(file: &Path, message: impl Into<String>) -> Violation {
        Violation::err(file.to_path_buf(), 0, 0, message.into())
    }
}
