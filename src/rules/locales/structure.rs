use crate::utils::{LinterJsonOps, ViolationReporterOps};
use crate::{JsonNodeKind, Violation};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub(super) type LocaleBaseline = (
    BTreeMap<String, JsonNodeKind>,
    BTreeMap<String, BTreeSet<String>>,
    BTreeMap<String, String>,
);

pub struct LocaleStructureOps;

impl LocaleStructureOps {
    pub fn build_locale_baseline(
        ja_path: &Path,
        en_path: &Path,
    ) -> Result<LocaleBaseline, Vec<Violation>> {
        let (ja_shape, ja_placeholders, _) = Self::read_locale(ja_path, false)?;
        let (en_shape, en_placeholders, en_values) = Self::read_locale(en_path, true)?;

        let mut violations = Self::compare_locale_shape(ja_path, &en_shape, &ja_shape);
        violations.extend(Self::compare_locale_shape(en_path, &ja_shape, &en_shape));
        violations.extend(Self::compare_locale_placeholders(
            ja_path,
            &en_shape,
            &en_placeholders,
            &ja_placeholders,
        ));
        violations.extend(Self::compare_locale_placeholders(
            en_path,
            &ja_shape,
            &ja_placeholders,
            &en_placeholders,
        ));
        if violations.is_empty() {
            Ok((ja_shape, ja_placeholders, en_values))
        } else {
            Err(violations)
        }
    }

    pub(super) fn read_locale(
        path: &Path,
        collect_values: bool,
    ) -> Result<LocaleBaseline, Vec<Violation>> {
        let document = LinterJsonOps::parse_json_file(path)?;
        let mut shape = BTreeMap::new();
        let mut placeholders = BTreeMap::new();
        let mut values = BTreeMap::new();
        LinterJsonOps::visit_paths(&document, None, &mut |path, node| {
            shape.insert(path.to_owned(), JsonNodeKind::from_value(node));
            if let Value::String(text) = node {
                placeholders.insert(path.to_owned(), LinterJsonOps::extract_placeholders(text));
                if collect_values {
                    values.insert(path.to_owned(), text.clone());
                }
            }
        });
        Ok((shape, placeholders, values))
    }

    pub fn compare_locale_shape(
        file: &Path,
        expected_shape: &BTreeMap<String, JsonNodeKind>,
        actual_shape: &BTreeMap<String, JsonNodeKind>,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();
        Self::check_missing_keys(file, expected_shape, actual_shape, &mut violations);
        Self::check_extra_keys(file, expected_shape, actual_shape, &mut violations);
        Self::check_kind_mismatches(file, expected_shape, actual_shape, &mut violations);
        violations
    }

    fn check_missing_keys(
        file: &Path,
        expected_shape: &BTreeMap<String, JsonNodeKind>,
        actual_shape: &BTreeMap<String, JsonNodeKind>,
        violations: &mut Vec<Violation>,
    ) {
        for missing in expected_shape
            .keys()
            .filter(|key| !actual_shape.contains_key(*key))
        {
            violations.push(ViolationReporterOps::locale_violation(
                file,
                format!("Missing locale key `{missing}` compared with ja.json/en.json."),
            ));
        }
    }

    fn check_extra_keys(
        file: &Path,
        expected_shape: &BTreeMap<String, JsonNodeKind>,
        actual_shape: &BTreeMap<String, JsonNodeKind>,
        violations: &mut Vec<Violation>,
    ) {
        for extra in actual_shape
            .keys()
            .filter(|key| !expected_shape.contains_key(*key))
        {
            violations.push(ViolationReporterOps::locale_violation(
                file,
                format!("Unexpected locale key `{extra}` not present in ja.json/en.json."),
            ));
        }
    }

    fn check_kind_mismatches(
        file: &Path,
        expected_shape: &BTreeMap<String, JsonNodeKind>,
        actual_shape: &BTreeMap<String, JsonNodeKind>,
        violations: &mut Vec<Violation>,
    ) {
        for (path, expected_kind) in expected_shape {
            let Some(actual_kind) = actual_shape.get(path) else {
                continue;
            };
            if actual_kind != expected_kind {
                violations.push(ViolationReporterOps::locale_violation(
                    file,
                    format!(
                        "Locale node kind mismatch at `{path}`: expected {expected_kind}, found {actual_kind}."
                    ),
                ));
            }
        }
    }

    pub fn compare_locale_placeholders(
        file: &Path,
        expected_shape: &BTreeMap<String, JsonNodeKind>,
        expected_placeholders: &BTreeMap<String, BTreeSet<String>>,
        actual_placeholders: &BTreeMap<String, BTreeSet<String>>,
    ) -> Vec<Violation> {
        let mut violations = Vec::new();

        for (path, kind) in expected_shape {
            if kind != &JsonNodeKind::String {
                continue;
            }

            let expected = expected_placeholders.get(path).cloned().unwrap_or_default();
            let actual = actual_placeholders.get(path).cloned().unwrap_or_default();

            if actual != expected {
                violations.push(ViolationReporterOps::locale_violation(
                    file,
                    format!(
                        "Locale placeholder mismatch at `{path}`: expected {:?}, found {:?}.",
                        expected, actual
                    ),
                ));
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests;
