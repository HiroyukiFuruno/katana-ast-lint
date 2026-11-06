use crate::Violation;
use crate::config::{RuleConfig, Severity};
use once_cell::sync::Lazy;
use std::path::Path;

pub mod coding;
pub mod i18n;
pub mod locales;
pub mod structure;

pub use coding::{
    CommentStyleOps, ConditionalFrameOps, ErrorFirstOps, FrameStrokeOps, HorizontalLayoutOps,
    IconButtonFillOps, LazyCodeOps, MagicNumberOps, MarkdownSandboxOps, MinRectSizingOps,
    PerformanceOps, ProcessCommandOps, ProhibitedAttributesOps, ProhibitedTypesOps,
    ScrollAreaInnerRectLeakOps,
};

pub use i18n::{I18nOps, IconOps};
pub use locales::LocaleOps;
pub use structure::{
    FileLengthOps, FunctionLengthOps, NestingDepthOps, PubFreeFnOps, TypeSeparationOps,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleCategory {
    Coding,
    Structure,
    I18n,
    Locales,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputKind {
    RustFile,
    Directory,
}

pub type LintFn = fn(&Path, &syn::File, &RuleConfig) -> Vec<Violation>;
pub type LintDirFn = fn(&Path) -> Vec<Violation>;

pub enum RuleImplementation {
    RustFile(LintFn),
    Directory(LintDirFn),
}

pub struct RuleDefinition {
    pub id: &'static str,
    pub category: RuleCategory,
    pub default_severity: Severity,
    pub hint: &'static str,
    pub implementation: RuleImplementation,
    pub default_threshold: Option<usize>,
}

impl RuleDefinition {
    pub(crate) fn lint_rust_file(
        &self,
        lint: LintFn,
        path: &Path,
        scoped_path: &Path,
        syntax: &syn::File,
        config: &RuleConfig,
    ) -> Vec<Violation> {
        match self.id {
            "file-length" => FileLengthOps::lint_scoped(path, scoped_path, syntax, config),
            "type-separation" => TypeSeparationOps::lint_scoped(path, scoped_path, syntax),
            "min-rect-sizing" => MinRectSizingOps::lint_scoped(path, scoped_path, syntax),
            _ => lint(path, syntax, config),
        }
    }
}

pub static RULE_CATALOG: Lazy<Vec<RuleDefinition>> =
    Lazy::new(|| {
        coding::catalog::RULES.into_iter().chain([
        /* WHY: Structure rules category. */
        RuleDefinition {
            id: "file-length",
            category: RuleCategory::Structure,
            default_severity: Severity::Error,
            hint: "Split files that exceed the responsibility boundary.",
            implementation: RuleImplementation::RustFile(FileLengthOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "function-length",
            category: RuleCategory::Structure,
            default_severity: Severity::Error,
            hint: "Extract helper methods when functions grow too large.",
            implementation: RuleImplementation::RustFile(FunctionLengthOps::lint_with_config),
            default_threshold: Some(30),
        },
        RuleDefinition {
            id: "nesting-depth",
            category: RuleCategory::Structure,
            default_severity: Severity::Error,
            hint: "Use early returns or extract helpers instead of deep nesting.",
            implementation: RuleImplementation::RustFile(NestingDepthOps::lint_with_config),
            default_threshold: Some(3),
        },
        RuleDefinition {
            id: "pub-free-fn",
            category: RuleCategory::Structure,
            default_severity: Severity::Error,
            hint: "Avoid public free-standing functions; prefer `impl` blocks or `pub(crate)`.",
            implementation: RuleImplementation::RustFile(PubFreeFnOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "type-separation",
            category: RuleCategory::Structure,
            default_severity: Severity::Error,
            hint: "Move public types to dedicated modules when implementation logic grows.",
            implementation: RuleImplementation::RustFile(TypeSeparationOps::lint_with_config),
            default_threshold: None,
        },
        /* WHY: I18n rules category. */
        RuleDefinition {
            id: "i18n",
            category: RuleCategory::I18n,
            default_severity: Severity::Error,
            hint: "Avoid hard-coded strings; use the i18n system.",
            implementation: RuleImplementation::RustFile(I18nOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "icon",
            category: RuleCategory::I18n,
            default_severity: Severity::Error,
            hint: "Use standard icons for UI elements.",
            implementation: RuleImplementation::RustFile(IconOps::lint_with_config),
            default_threshold: None,
        },
        /* WHY: Locales rules category. */
        RuleDefinition {
            id: "locales",
            category: RuleCategory::Locales,
            default_severity: Severity::Error,
            hint: "Ensure locale files are consistent and complete.",
            implementation: RuleImplementation::Directory(LocaleOps::lint),
            default_threshold: None,
        },
    ]).collect()
    });
