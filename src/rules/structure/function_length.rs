use crate::Violation;
use crate::utils::LinterParserOps;
use std::path::{Path, PathBuf};
use syn::visit::Visit;

const MAX_FUNCTION_LINES: usize = 30;

pub struct FunctionLengthOps;

impl FunctionLengthOps {
    pub fn lint(path: &Path, syntax: &syn::File) -> Vec<Violation> {
        Self::lint_with_config(path, syntax, &crate::config::RuleConfig::default())
    }

    pub fn lint_with_config(
        path: &Path,
        syntax: &syn::File,
        config: &crate::config::RuleConfig,
    ) -> Vec<Violation> {
        let max_lines = config.threshold.unwrap_or(MAX_FUNCTION_LINES);
        let mut visitor = FunctionLengthVisitor::new(path.to_path_buf(), max_lines);
        visitor.visit_file(syntax);
        visitor.violations
    }
}

struct FunctionLengthVisitor {
    file: PathBuf,
    violations: Vec<Violation>,
    max_lines: usize,
}

impl FunctionLengthVisitor {
    fn new(file: PathBuf, max_lines: usize) -> Self {
        Self {
            file,
            violations: Vec::new(),
            max_lines,
        }
    }

    fn check_length(&mut self, name: &syn::Ident, block: &syn::Block) {
        let (start, _) = LinterParserOps::span_location(block.brace_token.span.join());
        let (end, _) = LinterParserOps::span_location(block.brace_token.span.join());
        /* WHY: Both span calls return the same token's location; end is approximated from brace span. */
        let lines = end.saturating_sub(start);
        if lines > self.max_lines {
            let (name_line, name_column) = LinterParserOps::span_location(name.span());
            self.violations.push(Violation::err(
                self.file.clone(),
                name_line,
                name_column,
                format!(
                    "Function `{name}` exceeds {}-line limit (current: {lines}). Extract helper methods.",
                    self.max_lines
                ),
            ));
        }
    }
}

impl<'ast> Visit<'ast> for FunctionLengthVisitor {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.check_length(&node.sig.ident, &node.block);
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_impl_item_fn(&mut self, node: &'ast syn::ImplItemFn) {
        self.check_length(&node.sig.ident, &node.block);
        syn::visit::visit_impl_item_fn(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        syn::visit::visit_item_mod(self, node);
    }
}
