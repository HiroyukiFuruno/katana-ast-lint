use crate::{JsonNodeKind, Violation};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

pub struct LinterJsonOps;

impl LinterJsonOps {
    pub fn parse_json_file(path: &Path) -> Result<Value, Vec<Violation>> {
        let source = std::fs::read_to_string(path).map_err(|err| {
            vec![Violation::err(
                path.to_path_buf(),
                0,
                0,
                format!("Locale file read error: {err}"),
            )]
        })?;

        serde_json::from_str(&source).map_err(|err| {
            vec![Violation::err(
                path.to_path_buf(),
                err.line(),
                err.column(),
                format!("Locale JSON parse error: {err}"),
            )]
        })
    }

    pub fn collect_json_shape(
        value: &Value,
        path: Option<&str>,
        out: &mut BTreeMap<String, JsonNodeKind>,
    ) {
        Self::visit_paths(value, path, &mut |path, node| {
            out.insert(path.to_owned(), JsonNodeKind::from_value(node));
        });
    }

    pub fn collect_json_values(
        value: &Value,
        path: Option<&str>,
        out: &mut BTreeMap<String, String>,
    ) {
        Self::visit_paths(value, path, &mut |path, node| {
            if let Value::String(text) = node {
                out.insert(path.to_owned(), text.clone());
            }
        });
    }

    pub fn collect_json_placeholders(
        value: &Value,
        path: Option<&str>,
        out: &mut BTreeMap<String, BTreeSet<String>>,
    ) {
        Self::visit_paths(value, path, &mut |path, node| {
            if let Value::String(text) = node {
                out.insert(path.to_owned(), Self::extract_placeholders(text));
            }
        });
    }

    pub(crate) fn visit_paths(
        value: &Value,
        path: Option<&str>,
        visit: &mut impl FnMut(&str, &Value),
    ) {
        if let Some(path) = path {
            visit(path, value);
        }
        match value {
            Value::Object(map) => {
                for (key, child) in map {
                    let child_path = path
                        .map(|prefix| format!("{prefix}.{key}"))
                        .unwrap_or_else(|| key.to_string());
                    Self::visit_paths(child, Some(&child_path), visit);
                }
            }
            Value::Array(items) => {
                for (index, child) in items.iter().enumerate() {
                    let child_path = path
                        .map(|prefix| format!("{prefix}[{index}]"))
                        .unwrap_or_else(|| format!("[{index}]"));
                    Self::visit_paths(child, Some(&child_path), visit);
                }
            }
            Value::String(_) | Value::Number(_) | Value::Bool(_) | Value::Null => {}
        }
    }

    pub fn extract_placeholders(text: &str) -> BTreeSet<String> {
        let mut placeholders = BTreeSet::new();
        let bytes = text.as_bytes();
        let mut start = 0usize;

        while start < bytes.len() {
            if bytes[start] != b'{' {
                start += 1;
                continue;
            }

            let Some(end_rel) = bytes[start + 1..].iter().position(|byte| *byte == b'}') else {
                break;
            };
            let end = start + 1 + end_rel;
            let candidate = &text[start + 1..end];
            if Self::is_placeholder_name(candidate) {
                placeholders.insert(candidate.to_string());
            }
            start = end + 1;
        }

        placeholders
    }

    pub fn is_placeholder_name(candidate: &str) -> bool {
        let mut chars = candidate.chars();
        let Some(first) = chars.next() else {
            return false;
        };

        (first.is_ascii_alphabetic() || first == '_')
            && chars.all(|char| char.is_ascii_alphanumeric() || char == '_')
    }
}

#[cfg(test)]
mod tests;
