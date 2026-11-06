use crate::Violation;
use crate::config::{AllowEntry, RuleConfig, Severity};
use crate::utils::{LinterFileOps, LinterParserOps};
use chrono::NaiveDate;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/* WHY: 同じ実行内のルールでだけ入力を共有し、次回は編集後のファイルを読み直す。 */
#[derive(Default)]
pub(crate) struct RustSources<'scope> {
    scope: Option<&'scope Path>,
    files_by_root: HashMap<PathBuf, Vec<PathBuf>>,
    parsed_files: HashMap<PathBuf, Result<syn::File, Vec<Violation>>>,
}

impl<'scope> RustSources<'scope> {
    pub(crate) fn new(scope: Option<&'scope Path>) -> Self {
        Self {
            scope,
            ..Self::default()
        }
    }

    pub(crate) fn lint_directory(
        &mut self,
        root: &Path,
        config: &RuleConfig,
        lint: impl Fn(&Path, Option<&Path>, &syn::File, &RuleConfig) -> Vec<Violation>,
        violations: &mut Vec<Violation>,
    ) {
        let files = self
            .files_by_root
            .entry(root.to_path_buf())
            .or_insert_with(|| LinterFileOps::collect_rs_files_in_scope(root, self.scope));
        let today = chrono::Utc::now().naive_utc().date();
        let severity = config.severity.unwrap_or(Severity::Error);

        for file in files.iter() {
            let scoped_path = LinterFileOps::path_in_scope(file, self.scope);
            if Self::is_allowed(file, scoped_path.unwrap_or(file), &config.allow, today) {
                continue;
            }

            match self
                .parsed_files
                .entry(file.clone())
                .or_insert_with(|| LinterParserOps::parse_file(file))
            {
                Ok(syntax) => {
                    let mut found = lint(file, scoped_path, syntax, config);
                    found
                        .iter_mut()
                        .for_each(|violation| violation.severity = severity);
                    violations.append(&mut found);
                }
                Err(errors) => violations.extend_from_slice(errors),
            }
        }
    }

    fn is_allowed(
        file: &Path,
        policy_path: &Path,
        entries: &[AllowEntry],
        today: NaiveDate,
    ) -> bool {
        entries
            .iter()
            .filter(|entry| {
                let path = if entry.path.is_absolute() {
                    file
                } else {
                    policy_path
                };
                path.ends_with(&entry.path)
            })
            .any(|entry| Self::allowance_is_current(entry, today))
    }

    fn allowance_is_current(entry: &AllowEntry, today: NaiveDate) -> bool {
        let Some(expiry) = &entry.expires_at else {
            return true;
        };
        match NaiveDate::parse_from_str(expiry, "%Y-%m-%d") {
            Ok(date) => today < date,
            Err(error) => {
                eprintln!("Warning: Failed to parse expiry date '{expiry}': {error}");
                false
            }
        }
    }
}
