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
        let mut violations = Vec::new();
        let Ok(content) = std::fs::read_to_string(path) else {
            return violations;
        };

        for (line_idx, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            /* WHY: Standard inline comments (//) are completely prohibited
            to save context window and enforce formal annotations. */
            if trimmed.starts_with("//")
                && !trimmed.starts_with("///")
                && !trimmed.starts_with("//!")
            {
                violations.push(Violation::err(path.to_path_buf(),
                    line_idx + 1,
                    1,
                    "Standard `//` comments are prohibited to save AI context. Use `/* WHY:`, `/// SAFETY:`, or `/// TODO:` for logic annotations.".to_string()));
            }

            /* WHY: If an annotation contains a justification keyword,
            it MUST strictly start with it to prevent cheating. */
            if trimmed.starts_with("///") || trimmed.starts_with("/*") {
                let text = if trimmed.starts_with("///") {
                    trimmed.trim_start_matches('/').trim()
                } else {
                    trimmed.trim_start_matches("/*").trim()
                };

                let contains_keyword =
                    text.contains("WHY:") || text.contains("SAFETY:") || text.contains("TODO:");
                let invalid_start = !text.starts_with("WHY:")
                    && !text.starts_with("SAFETY:")
                    && !text.starts_with("TODO:");

                if contains_keyword && invalid_start {
                    violations.push(Violation::err(path.to_path_buf(),
                        line_idx + 1,
                        1,
                        "Annotations containing WHY/SAFETY/TODO must strictly start with the keyword.".to_string()));
                }
            }
        }

        violations
    }
}
