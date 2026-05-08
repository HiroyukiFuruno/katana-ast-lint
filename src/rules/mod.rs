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

pub static RULE_CATALOG: Lazy<Vec<RuleDefinition>> = Lazy::new(|| {
    vec![
        /* WHY: Coding rules category. */
        RuleDefinition {
            id: "comment-style",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Use `//` for internal comments and `///` for documentation.",
            implementation: RuleImplementation::RustFile(CommentStyleOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "conditional-frame",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Prefer `ui.add_visible_ui` or `ui.set_visible` over conditional `ui.frame` calls.",
            implementation: RuleImplementation::RustFile(ConditionalFrameOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "error-first",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Use early returns for error handling to reduce nesting.",
            implementation: RuleImplementation::RustFile(ErrorFirstOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "frame-stroke",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Use `Frame::stroke` for borders instead of manual `Rect` drawing.",
            implementation: RuleImplementation::RustFile(FrameStrokeOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "horizontal-layout",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Use `ui.horizontal` for horizontal item placement.",
            implementation: RuleImplementation::RustFile(HorizontalLayoutOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "icon-button-fill",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Ensure `IconButton` has consistent fill behavior.",
            implementation: RuleImplementation::RustFile(IconButtonFillOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "lazy-code",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Remove `todo!()`, `unimplemented!()`, and `dbg!()` macros.",
            implementation: RuleImplementation::RustFile(LazyCodeOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "magic-numbers",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Extract numeric literals to named constants.",
            implementation: RuleImplementation::RustFile(MagicNumberOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "markdown-sandbox",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Ensure markdown snippets are properly sandboxed.",
            implementation: RuleImplementation::RustFile(MarkdownSandboxOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "min-rect-sizing",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Use `ui.allocate_at_least` for minimum size constraints.",
            implementation: RuleImplementation::RustFile(MinRectSizingOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "performance",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Avoid expensive operations in UI update loops.",
            implementation: RuleImplementation::RustFile(PerformanceOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "process-command",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Use shared process utilities instead of raw `std::process::Command`.",
            implementation: RuleImplementation::RustFile(ProcessCommandOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "prohibited-attributes",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Remove `#[allow(dead_code)]`; delete unused code instead.",
            implementation: RuleImplementation::RustFile(ProhibitedAttributesOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "prohibited-types",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Use ecosystem-preferred types for common data structures.",
            implementation: RuleImplementation::RustFile(ProhibitedTypesOps::lint_with_config),
            default_threshold: None,
        },
        RuleDefinition {
            id: "scrollarea-inner-rect-leak",
            category: RuleCategory::Coding,
            default_severity: Severity::Error,
            hint: "Prevent inner rect information from leaking outside `ScrollArea`.",
            implementation: RuleImplementation::RustFile(
                ScrollAreaInnerRectLeakOps::lint_with_config,
            ),
            default_threshold: None,
        },
        /* WHY: Structure rules category. */
        RuleDefinition {
            id: "file-length",
            category: RuleCategory::Structure,
            default_severity: Severity::Error,
            hint: "Split files that exceed the responsibility boundary.",
            implementation: RuleImplementation::RustFile(FileLengthOps::lint_with_config),
            default_threshold: Some(200),
        },
        RuleDefinition {
            id: "function-length",
            category: RuleCategory::Structure,
            default_severity: Severity::Error,
            hint: "Extract helper methods when functions grow too large.",
            implementation: RuleImplementation::RustFile(FunctionLengthOps::lint_with_config),
            default_threshold: Some(50),
        },
        RuleDefinition {
            id: "nesting-depth",
            category: RuleCategory::Structure,
            default_severity: Severity::Error,
            hint: "Use early returns or extract helpers instead of deep nesting.",
            implementation: RuleImplementation::RustFile(NestingDepthOps::lint_with_config),
            default_threshold: Some(4),
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
    ]
});
