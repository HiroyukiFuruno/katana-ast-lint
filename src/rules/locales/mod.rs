use crate::Violation;
use std::path::Path;

pub mod audit;
pub mod catalog;
pub mod discovery;
pub mod structure;
pub mod values;

use catalog::LocaleCatalogOps;
use discovery::LocaleDiscoveryOps;
use structure::{LocaleBaseline, LocaleStructureOps};
use values::LocaleValueOps;

pub use audit::LocaleAuditOps as LocaleAudit;

pub struct LocaleOps;

impl LocaleOps {
    pub fn lint(locale_dir: &Path) -> Vec<Violation> {
        let locale_files = match Self::get_locale_files_or_error(locale_dir) {
            Ok(files) => files,
            Err(v) => return v,
        };

        let language_codes = match LocaleCatalogOps::parse_languages_catalog(locale_dir) {
            Ok(codes) => codes,
            Err(violations) => return violations,
        };
        let mut all_violations =
            LocaleCatalogOps::compare_languages_catalog(locale_dir, &locale_files, &language_codes);

        match LocaleStructureOps::build_locale_baseline(
            &locale_dir.join("ja.json"),
            &locale_dir.join("en.json"),
        ) {
            Ok(baseline) => {
                for file in locale_files {
                    all_violations.extend(Self::lint_translation(&file, &baseline));
                }
            }
            Err(violations) => all_violations.extend(violations),
        }

        all_violations
    }

    fn get_locale_files_or_error(
        locale_dir: &Path,
    ) -> Result<Vec<std::path::PathBuf>, Vec<Violation>> {
        let locale_files = LocaleDiscoveryOps::collect_locale_json_files(locale_dir);
        if locale_files.is_empty() {
            return Err(vec![crate::utils::ViolationReporterOps::locale_violation(
                locale_dir,
                format!(
                    "No locale JSON files found for analysis: {}",
                    locale_dir.display()
                ),
            )]);
        }
        Ok(locale_files)
    }

    fn lint_translation(file: &Path, baseline: &LocaleBaseline) -> Vec<Violation> {
        if file.ends_with("ja.json") || file.ends_with("en.json") {
            return Vec::new();
        }
        let (shape, placeholders, values) = match LocaleStructureOps::read_locale(file, true) {
            Ok(locale) => locale,
            Err(violations) => return violations,
        };
        let (baseline_shape, baseline_placeholders, en_values) = baseline;
        let mut violations = LocaleStructureOps::compare_locale_shape(file, baseline_shape, &shape);
        violations.extend(LocaleStructureOps::compare_locale_placeholders(
            file,
            baseline_shape,
            baseline_placeholders,
            &placeholders,
        ));
        violations.extend(LocaleValueOps::compare_locale_values(
            file, en_values, &values,
        ));
        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lint_locale_files_reports_empty_directory() {
        let tmp = tempfile::TempDir::new().unwrap();
        let violations = LocaleOps::lint(tmp.path());
        assert_eq!(violations.len(), 1);
        assert!(
            violations[0]
                .message
                .contains("No locale JSON files found for analysis")
        );
    }
}
