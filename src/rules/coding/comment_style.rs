use crate::Violation;
use std::path::Path;

pub struct CommentStyleOps;

impl CommentStyleOps {
    pub fn lint(path: &Path, syntax: &syn::File) -> Vec<Violation> {
        Self::lint_with_config(path, syntax, &crate::config::RuleConfig::default())
    }

    pub fn lint_with_config(
        path: &Path,
        _syntax: &syn::File,
        _config: &crate::config::RuleConfig,
    ) -> Vec<Violation> {
        let Ok(content) = std::fs::read_to_string(path) else {
            return Vec::new();
        };

        content
            .lines()
            .enumerate()
            .filter_map(|(index, line)| {
                let message = Self::line_violation(line.trim())?;
                Some(Violation::err(path.to_path_buf(), index + 1, 1, message))
            })
            .collect()
    }

    fn line_violation(line: &str) -> Option<&'static str> {
        if line.starts_with("//") && !line.starts_with("///") && !line.starts_with("//!") {
            return Some(
                "Standard `//` comments are prohibited to save AI context. Use `/* WHY:`, `/// SAFETY:`, or `/// TODO:` for logic annotations.",
            );
        }
        let text = if line.starts_with("///") {
            line.trim_start_matches('/').trim()
        } else if line.starts_with("/*") {
            line.trim_start_matches("/*").trim()
        } else {
            return None;
        };

        /* WHY: 注釈の途中にキーワードを書いて規約を回避することを防ぐ。 */
        let keywords = ["WHY:", "SAFETY:", "TODO:"];
        let contains_keyword = keywords.iter().any(|keyword| text.contains(keyword));
        let starts_with_keyword = keywords.iter().any(|keyword| text.starts_with(keyword));
        (contains_keyword && !starts_with_keyword).then_some(
            "Annotations containing WHY/SAFETY/TODO must strictly start with the keyword.",
        )
    }
}
