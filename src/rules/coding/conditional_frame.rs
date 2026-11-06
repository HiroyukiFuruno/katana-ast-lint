use crate::Violation;
use crate::utils::LinterParserOps;
use std::path::{Path, PathBuf};
use syn::visit::Visit;

/* WHY: Detects egui widgets that conditionally show a frame only on hover.

 These widgets cause layout jitter because their size changes between
 inactive and hovered states:

 Pattern that triggers this rule:
 ```ignore
    VIOLATION: frame appears only on hover
    ui.selectable_label(false, "Text");
 ```
*/
pub struct ConditionalFrameOps;

impl ConditionalFrameOps {
    pub fn lint(path: &Path, syntax: &syn::File) -> Vec<Violation> {
        Self::lint_with_config(path, syntax, &crate::config::RuleConfig::default())
    }

    pub fn lint_with_config(
        path: &Path,
        syntax: &syn::File,
        _config: &crate::config::RuleConfig,
    ) -> Vec<Violation> {
        Self::lint_scoped(path, path, syntax)
    }

    pub(crate) fn lint_scoped(
        path: &Path,
        scoped_path: &Path,
        syntax: &syn::File,
    ) -> Vec<Violation> {
        /* WHY: widgets/menu_button/mod.rs is the canonical wrapper around ui.menu_button().
        All other call-sites must use MenuButtonOps::show() so egui's conditional-frame
        behavior is confined behind a single abstraction boundary. */
        if scoped_path
            .to_string_lossy()
            .replace('\\', "/")
            .contains("widgets/menu_button/mod.rs")
        {
            return Vec::new();
        }

        let source = std::fs::read_to_string(path).unwrap_or_default();
        let lines: Vec<&str> = source.lines().collect();

        let mut visitor = ConditionalFrameVisitor {
            file_path: path.to_path_buf(),
            source_lines: lines,
            violations: Vec::new(),
        };
        visitor.visit_file(syntax);
        visitor.violations
    }
}

struct ConditionalFrameVisitor<'a> {
    file_path: PathBuf,
    source_lines: Vec<&'a str>,
    violations: Vec<Violation>,
}

impl ConditionalFrameVisitor<'_> {
    fn is_suppressed(&self, line: usize) -> bool {
        if line > 1
            && let Some(prev) = self.source_lines.get(line - 2)
        {
            let trimmed = prev.trim();
            return trimmed.starts_with("/// WHY: allow(conditional_frame)")
                || trimmed.starts_with("/* WHY: allow(conditional_frame)");
        }
        false
    }
}

impl ConditionalFrameVisitor<'_> {
    fn remediation(method: &syn::Ident) -> Option<&'static str> {
        match method.to_string().as_str() {
            "selectable_label" => Some(
                "Use `Button::selectable(selected, text).frame_when_inactive(true)` to prevent hover-jitter.",
            ),
            "selectable_value" => Some(
                "Use `Button::selectable(val == variant, text).frame_when_inactive(true)` to prevent hover-jitter.",
            ),
            "menu_button" => Some(
                "menu_button shows a frame only on hover, causing layout jitter. Wrap in a custom button with stable frame.",
            ),
            _ => None,
        }
    }
}

impl<'ast> Visit<'ast> for ConditionalFrameVisitor<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if let Some(message) = Self::remediation(&node.method) {
            let (line, column) = LinterParserOps::span_location(node.method.span());
            if !self.is_suppressed(line) {
                self.violations.push(Violation::err(
                    self.file_path.clone(),
                    line,
                    column,
                    message,
                ));
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}
