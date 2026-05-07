use crate::Violation;
use std::path::Path;

pub struct LinterParserOps;

impl LinterParserOps {
    pub fn has_cfg_test_attr(attrs: &[syn::Attribute]) -> bool {
        attrs.iter().any(|attr| {
            if attr.path().is_ident("cfg") {
                let Ok(syn::Meta::Path(path)) = attr.parse_args::<syn::Meta>() else {
                    return false;
                };
                return path.is_ident("test");
            }
            attr.path().is_ident("test")
        })
    }

    pub fn is_allowed_number(value: f64) -> bool {
        (value - 0.0).abs() < f64::EPSILON
            || (value - 1.0).abs() < f64::EPSILON
            || (value - 2.0).abs() < f64::EPSILON
            || (value - 100.0).abs() < f64::EPSILON
            || (value - (-1.0)).abs() < f64::EPSILON
    }

    pub fn span_location(span: proc_macro2::Span) -> (usize, usize) {
        (span.start().line, span.start().column + 1)
    }

    pub fn parse_file(path: &Path) -> Result<syn::File, Vec<Violation>> {
        let source = std::fs::read_to_string(path).map_err(|err| {
            vec![Violation::err(
                path.to_path_buf(),
                0,
                0,
                format!("Rust file read error: {err}"),
            )]
        })?;

        syn::parse_file(&source).map_err(|err| {
            let (line, column) = Self::span_location(err.span());
            vec![Violation::err(
                path.to_path_buf(),
                line,
                column,
                format!("Syntax parse error: {err}"),
            )]
        })
    }

    pub fn is_allowed_string(s: &str) -> bool {
        let trimmed = s.trim();

        if trimmed.is_empty() {
            return true;
        }

        let chars: Vec<char> = trimmed.chars().collect();
        if chars.len() == 1 {
            let c = chars[0];
            if !c.is_ascii_alphabetic() {
                return true;
            }
            if c == 'x' || c == 'X' {
                return true;
            }
            return false;
        }

        if trimmed
            .chars()
            .all(|c| !c.is_alphabetic() || Self::is_emoji_or_symbol(c))
        {
            return true;
        }

        false
    }

    pub fn is_emoji_or_symbol(c: char) -> bool {
        matches!(c,
            '\u{2000}'..='\u{2BFF}'
            | '\u{2E00}'..='\u{2E7F}'
            | '\u{3000}'..='\u{303F}'
            | '\u{FE00}'..='\u{FE0F}'
            | '\u{FE30}'..='\u{FE4F}'
            | '\u{1F000}'..='\u{1FAFF}'
            | '\u{E0000}'..='\u{E007F}'
        )
    }
}

#[cfg(test)]
mod tests;
