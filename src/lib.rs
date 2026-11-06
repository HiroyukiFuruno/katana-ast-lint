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

mod ast_linter;
mod runner;
mod rust_sources;

pub mod config;
pub mod rules;
pub mod utils;

use crate::config::Severity;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

pub use ast_linter::AstLinterOps;
pub use runner::KatanaAstLint;

/* WHY: Domain entities for linter violation reporting and JSON AST traversal. */
#[non_exhaustive]
#[derive(Debug)]
pub enum KalRunError {
    /// rule violations were detected and reported
    Violations(String),
    /// kal.json or workspace configuration is invalid / unreadable
    Configuration(String),
    /// non-violation runtime failure (IO, internal invariants)
    System(String),
}

impl std::fmt::Display for KalRunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Violations(msg) | Self::Configuration(msg) | Self::System(msg) => {
                write!(f, "{msg}")
            }
        }
    }
}

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
