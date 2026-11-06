use crate::Violation;
use crate::utils::{LinterFileOps, LinterJsonOps, ViolationReporterOps};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

/* WHY: Grandfathered duplicates that existed before this rule was introduced.
 * Each entry is the English value that is intentionally shared across sections.
 * To fix: consolidate into `common` and remove from this list. */
#[rustfmt::skip]
const GRANDFATHERED_DUPLICATES: &[&str] = &[
    "Settings", "Workspace", "Refresh Document", "Toggle Slideshow",
    "Use Workspace-Local Configuration", "Zoom Out", "Zoom In",
    "Check for Updates", "Official Website", "Behavior",
    "Modified", "Open Workspace", "Markdown Document",
    "Delete", "Open File", "Open Folder", "Pinned", "Name",
    "Refresh", "Reset", "Create File", "Create Folder",
    "Close", "Save", "Undo", "Redo", "Copy", "Paste", "Cut",
];

/* WHY: Strings shorter than this threshold are typically single words that
 * appear naturally across sections (e.g. "Version", "Layout") and would
 * generate too much noise. 8 characters filters those out cleanly. */
const MIN_DUPLICATE_VALUE_LEN: usize = 8;

pub struct LocaleAuditOps;

impl LocaleAuditOps {
    /// Rule: i18n-unused-keys
    ///
    /// Scans en.json and checks whether each leaf key is referenced in Rust source files
    /// under `crates/`. Keys with no usage should be removed.
    /// A key `section.key` is considered used if any `.rs` file contains the string
    /// `section.key` or just the leaf name (e.g. `key`).
    pub fn lint_unused_keys(workspace_root: &Path, locale_dir: &Path) -> Vec<Violation> {
        let en_path = locale_dir.join("en.json");
        let Ok(en_json) = LinterJsonOps::parse_json_file(&en_path) else {
            return Vec::new();
        };
        let mut leaves = BTreeMap::new();
        LinterJsonOps::collect_json_values(&en_json, None, &mut leaves);

        let corpus =
            LinterFileOps::collect_files_by_extension(&workspace_root.join("crates"), "rs")
                .iter()
                .filter_map(|path| std::fs::read_to_string(path).ok())
                .collect::<Vec<_>>()
                .join("\n");

        leaves
            .keys()
            .filter(|key| !Self::is_referenced(key, &corpus))
            .map(|key| {
                ViolationReporterOps::locale_violation(
                    &en_path,
                    format!(
                        "Locale key `{key}` is not referenced in any Rust source file. \
                     Consider removing it to keep translations lean."
                    ),
                )
            })
            .collect()
    }

    fn is_referenced(key: &str, corpus: &str) -> bool {
        /* WHY: Rust コードは翻訳キーの全体、末尾2要素、末尾1要素を参照できる。 */
        let section_leaf = key
            .rmatch_indices('.')
            .nth(1)
            .map_or(key, |(index, _)| &key[index + 1..]);
        let leaf = key.rsplit('.').next().unwrap_or_default();
        corpus.contains(key) || corpus.contains(section_leaf) || corpus.contains(leaf)
    }

    /// Rule: i18n-duplicate-values
    ///
    /// Detects strings shared across top-level sections in a locale file.
    pub fn lint_duplicate_values_within_file(locale_file: &Path) -> Vec<Violation> {
        let Ok(json) = LinterJsonOps::parse_json_file(locale_file) else {
            return Vec::new();
        };
        let mut leaves = BTreeMap::new();
        LinterJsonOps::collect_json_values(&json, None, &mut leaves);

        Self::duplicate_candidates(&leaves)
            .into_iter()
            .filter(|(_, keys)| Self::spans_sections(keys))
            .map(|(value, keys)| Self::duplicate_violation(locale_file, value, &keys))
            .collect()
    }

    fn duplicate_candidates(leaves: &BTreeMap<String, String>) -> HashMap<&str, Vec<&str>> {
        let common_values: HashSet<_> = leaves
            .iter()
            .filter(|(key, _)| key.starts_with("common."))
            .map(|(_, value)| value.as_str())
            .collect();

        let mut candidates = HashMap::<_, Vec<_>>::new();
        for (key, value) in leaves {
            if !Self::is_exempt_value(value, &common_values) {
                candidates
                    .entry(value.as_str())
                    .or_default()
                    .push(key.as_str());
            }
        }
        candidates
    }

    fn is_exempt_value(value: &str, common_values: &HashSet<&str>) -> bool {
        value.len() < MIN_DUPLICATE_VALUE_LEN
            || value
                .chars()
                .all(|c| c.is_ascii_digit() || c == '.' || c == '-')
            || value.starts_with('{')
            || common_values.contains(value)
            || GRANDFATHERED_DUPLICATES.contains(&value)
    }

    fn spans_sections(keys: &[&str]) -> bool {
        let Some((first, rest)) = keys.split_first() else {
            return false;
        };
        let first_section = first.split('.').next();
        rest.iter()
            .any(|key| key.split('.').next() != first_section)
    }

    fn duplicate_violation(file: &Path, value: &str, keys: &[&str]) -> Violation {
        let key_list = keys.join("`, `");
        ViolationReporterOps::locale_violation(
            file,
            format!(
                "Locale value \"{value}\" appears in multiple sections under keys: \
                 `{key_list}`. Consider consolidating into `common` to avoid \
                 maintaining the same translation in multiple places."
            ),
        )
    }
}
