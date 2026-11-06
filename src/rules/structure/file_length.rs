use crate::Violation;
use crate::config::RuleConfig;
use crate::utils::LinterParserOps;
use std::path::Path;
use syn::spanned::Spanned;
use syn::visit::Visit;

const MAX_SOURCE_LINES: usize = 200;
const MAX_TEST_LINES: usize = 300;

pub struct FileLengthOps;

impl FileLengthOps {
    pub fn lint(path: &Path, syntax: &syn::File) -> Vec<Violation> {
        Self::lint_with_config(path, syntax, &RuleConfig::default())
    }

    /// Counts source lines using module locations from the AST of the file at `path`.
    pub fn lint_with_config(
        path: &Path,
        syntax: &syn::File,
        config: &RuleConfig,
    ) -> Vec<Violation> {
        Self::lint_scoped(path, path, syntax, config)
    }

    pub(crate) fn lint_scoped(
        path: &Path,
        scoped_path: &Path,
        syntax: &syn::File,
        config: &RuleConfig,
    ) -> Vec<Violation> {
        let Ok(source) = std::fs::read_to_string(path) else {
            return Vec::new();
        };
        let path_str = scoped_path.to_string_lossy().replace('\\', "/");
        let is_test = path_str.contains("/tests/") || path_str.ends_with("tests.rs");
        let max_allowed = config.threshold.unwrap_or(if is_test {
            MAX_TEST_LINES
        } else {
            MAX_SOURCE_LINES
        });

        let mut modules = TestModuleLines {
            lines: source.trim_start_matches('\u{feff}').lines().collect(),
            excluded: 0,
        };
        modules.visit_file(syntax);
        let lines = modules.lines.len().saturating_sub(modules.excluded);
        if lines <= max_allowed {
            return Vec::new();
        }

        vec![Violation::err(
            path.to_path_buf(),
            1,
            1,
            format!(
                "File exceeds {max_allowed}-line limit (current: {lines}, excluding tests). Treat this as a responsibility-boundary failure: split by cohesive concerns, not by moving arbitrary lines."
            ),
        )]
    }
}

struct TestModuleLines<'source> {
    lines: Vec<&'source str>,
    excluded: usize,
}

impl TestModuleLines<'_> {
    fn exclude_module(&mut self, module: &syn::ItemMod) {
        let start = module.span().start();
        let end = module.span().end();
        if start >= end || start.line == 0 {
            return;
        }
        let (Some(first), Some(last)) = (
            self.lines.get(start.line.saturating_sub(1)),
            self.lines.get(end.line.saturating_sub(1)),
        ) else {
            return;
        };

        /* WHY: モジュールの前後に本番コードがある行は行数に含める。 */
        let starts_line = first.chars().take(start.column).all(char::is_whitespace);
        let ends_line = last.chars().skip(end.column).all(char::is_whitespace);
        let first_excluded = start.line.saturating_sub(usize::from(starts_line));
        let end_excluded = end.line.saturating_sub(usize::from(!ends_line));
        self.excluded += end_excluded.saturating_sub(first_excluded);
    }
}

impl<'ast> Visit<'ast> for TestModuleLines<'_> {
    fn visit_item_mod(&mut self, module: &'ast syn::ItemMod) {
        if module.attrs.iter().any(LinterParserOps::is_cfg_test_attr) {
            self.exclude_module(module);
        } else {
            syn::visit::visit_item_mod(self, module);
        }
    }
}
