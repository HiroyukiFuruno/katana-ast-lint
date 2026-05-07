use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct KalConfig {
    #[serde(default)]
    pub source_roots: Vec<PathBuf>,
    #[serde(default)]
    pub rules: HashMap<String, RuleConfig>,
    #[serde(default)]
    pub reporter: ReporterConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuleConfig {
    pub enabled: Option<bool>,
    pub severity: Option<Severity>,
    pub threshold: Option<usize>,
    pub reason: Option<String>,
    #[serde(default)]
    pub allow: Vec<AllowEntry>,
    #[serde(default)]
    pub inputs: Vec<PathBuf>,
}

impl RuleConfig {
    pub fn default_for_rule(rule_name: &str) -> Self {
        let enabled = !(rule_name == "i18n" || rule_name == "icon" || rule_name == "locales");

        Self {
            enabled: Some(enabled),
            severity: Some(Severity::Error),
            threshold: None,
            reason: None,
            allow: Vec::new(),
            inputs: Vec::new(),
        }
    }
}

impl Default for RuleConfig {
    fn default() -> Self {
        Self {
            enabled: Some(true),
            severity: Some(Severity::Error),
            threshold: None,
            reason: None,
            allow: Vec::new(),
            inputs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    #[default]
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllowEntry {
    pub path: PathBuf,
    pub reason: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ReporterConfig {
    pub mode: Option<ReporterMode>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReporterMode {
    #[default]
    Text,
    Json,
}

impl KalConfig {
    pub fn load_from_path(path: &std::path::Path) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file {}: {}", path.display(), e))?;
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse config file {}: {}", path.display(), e))
    }

    pub fn load_default() -> Self {
        Self::default()
    }
}
