use crate::Violation;
use crate::utils::LinterParserOps;
use std::path::{Path, PathBuf};
use syn::visit::Visit;

/* WHY: Detects border/stroke patterns that can cause layout jitter on hover. */
///
/// In egui, strokes on `Frame` or `painter().rect_stroke()` add pixels OUTSIDE
/// the element's rect. Without expansion compensation (box-sizing: border-box),
/// hover-time borders cause adjacent elements to shift.
///
/// Allowed patterns: theme_bridge (centralized expansion config), explicit suppression.
pub struct FrameStrokeOps;

impl FrameStrokeOps {
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
        /* WHY: theme_bridge は枠線の外側への拡張を一元管理する。 */
        if scoped_path
            .to_str()
            .is_some_and(|p| p.contains("theme_bridge"))
        {
            return Vec::new();
        }
        let mut visitor = FrameStrokeVisitor {
            file_path: path.to_path_buf(),
            violations: Vec::new(),
        };
        visitor.visit_file(syntax);
        visitor.violations
    }
}

struct FrameStrokeVisitor {
    file_path: PathBuf,
    violations: Vec<Violation>,
}

impl<'ast> Visit<'ast> for FrameStrokeVisitor {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let method_name = node.method.to_string();

        /* WHY: painter().rect_stroke() adds pixels outside the rect without expansion */
        /* WHY: compensation, causing border inflation on hover. Detect and require shrink(). */
        if method_name == "rect_stroke" {
            let (line, column) = LinterParserOps::span_location(node.method.span());
            self.violations.push(Violation::err(self.file_path.clone(),
                line,
                column,
                "Use `rect_filled` or compensate stroke width with `rect.shrink(stroke_width)` before `rect_stroke`.".to_string()));
        }

        /* WHY: .stroke() on a Frame builder adds pixels outside the widget rect. Detect this */
        /* WHY: by walking the receiver chain for Frame builder method names (fill/margin etc). */
        if method_name == "stroke" && !node.args.is_empty() && Self::is_frame_context(node) {
            let (line, column) = LinterParserOps::span_location(node.method.span());
            self.violations.push(Violation::err(self.file_path.clone(),
                line,
                column,
                "Frame `.stroke()` can cause layout jitter on hover. Use theme visuals (expansion-compensated) or wrap in a widget.".to_string()));
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}

impl FrameStrokeVisitor {
    /// Heuristic: walk the receiver chain to see if this `.stroke()` is called
    /// on a `Frame` builder (e.g., `Frame::NONE.fill(...).stroke(...)`).
    fn is_frame_context(node: &syn::ExprMethodCall) -> bool {
        let mut expr = node.receiver.as_ref();
        loop {
            expr = match expr {
                syn::Expr::MethodCall(inner) if Self::is_frame_builder_method(&inner.method) => {
                    &inner.receiver
                }
                syn::Expr::Path(path) => return Self::is_frame_path(&path.path),
                syn::Expr::Field(field) => {
                    if let syn::Member::Named(ident) = &field.member
                        && (ident == "NONE" || ident == "frame")
                    {
                        return true;
                    }
                    &field.base
                }
                syn::Expr::Call(call) => {
                    return matches!(&*call.func, syn::Expr::Path(path) if Self::is_frame_path(&path.path));
                }
                _ => return false,
            };
        }
    }

    fn is_frame_builder_method(method: &syn::Ident) -> bool {
        matches!(
            method.to_string().as_str(),
            "fill" | "inner_margin" | "outer_margin" | "corner_radius" | "rounding" | "shadow"
        )
    }

    fn is_frame_path(path: &syn::Path) -> bool {
        path.segments
            .iter()
            .any(|segment| segment.ident.to_string().contains("Frame"))
    }
}
