use crate::utils::{LinterJsonOps, ViolationReporterOps};
use crate::{JsonNodeKind, Violation};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

type LocaleBaseline = (
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
        match LinterJsonOps::parse_json_file(ja_path)
            .and_then(|ja| LinterJsonOps::parse_json_file(en_path).map(|en| (ja, en)))
        {
            Ok((ja_val, en_val)) => {
                let mut ja_sh = BTreeMap::new();
                let mut en_sh = BTreeMap::new();
                LinterJsonOps::collect_json_shape(&ja_val, None, &mut ja_sh);
                LinterJsonOps::collect_json_shape(&en_val, None, &mut en_sh);

                let mut v = Self::compare_locale_shape(ja_path, &en_sh, &ja_sh);
                v.extend(Self::compare_locale_shape(en_path, &ja_sh, &en_sh));

                let mut ja_pl = BTreeMap::new();
                let mut en_pl = BTreeMap::new();
                LinterJsonOps::collect_json_placeholders(&ja_val, None, &mut ja_pl);
                LinterJsonOps::collect_json_placeholders(&en_val, None, &mut en_pl);

                let mut en_values = BTreeMap::new();
                LinterJsonOps::collect_json_values(&en_val, None, &mut en_values);

                Self::validate_baseline_placeholders(
                    ja_path, en_path, &mut v, &ja_sh, &en_sh, &ja_pl, &en_pl,
                );

                if v.is_empty() {
                    Ok((ja_sh, ja_pl, en_values))
                } else {
                    Err(v)
                }
            }
            Err(e) => Err(e),
        }
    }

    fn validate_baseline_placeholders(
        ja_path: &Path,
        en_path: &Path,
        violations: &mut Vec<Violation>,
        ja_shape: &BTreeMap<String, JsonNodeKind>,
        en_shape: &BTreeMap<String, JsonNodeKind>,
        ja_placeholders: &BTreeMap<String, BTreeSet<String>>,
        en_placeholders: &BTreeMap<String, BTreeSet<String>>,
    ) {
        violations.extend(Self::compare_locale_placeholders(
            ja_path,
            en_shape,
            en_placeholders,
            ja_placeholders,
        ));
        violations.extend(Self::compare_locale_placeholders(
            en_path,
            ja_shape,
            ja_placeholders,
            en_placeholders,
        ));
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
