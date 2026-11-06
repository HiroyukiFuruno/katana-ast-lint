use crate::Violation;
use crate::utils::{LinterParserOps, ViolationReporterOps};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::BTreeMap;
use std::path::Path;

static EMOJI_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\p{Emoji_Presentation}").unwrap());

struct LocaleException {
    key: &'static str,
    value: &'static str,
}

/* WHY: Exclusion list for translation value validation.
1. Add identical values across languages (proper nouns, versions, etc.).
2. Use `*` for broad matching.
3. Meaningful words ("File", "Search") MUST NOT be excluded. */
#[rustfmt::skip]
const LOCALE_VALUE_EXCEPTIONS: &[LocaleException] = &[
    LocaleException {
        key: "rust",
        value: "Rust",
    },
    LocaleException {
        key: "support",
        value: "Support",
    },
    LocaleException {
        key: "action_close",
        value: "OK",
    },
    LocaleException {
        key: "key",
        value: "*",
    },
    LocaleException {
        key: "kind",
        value: "*",
    },
    LocaleException {
        key: "*",
        value: "KatanA",
    },
    LocaleException {
        key: "*",
        value: "PlantUML",
    },
    LocaleException {
        key: "*",
        value: "wkhtmltopdf",
    },
    LocaleException {
        key: "*",
        value: "Rust",
    },
    LocaleException {
        key: "render_error",
        value: "*",
    },
    LocaleException {
        key: "*",
        value: "Copyright",
    },
    LocaleException {
        key: "*",
        value: "Runtime",
    },
    LocaleException {
        key: "*",
        value: "Build",
    },
    LocaleException {
        key: "*",
        value: "Version",
    },
    LocaleException {
        key: "*",
        value: "v1.0.0",
    },
    LocaleException {
        key: "*",
        value: "Version: {version}",
    },
    LocaleException {
        key: "*",
        value: "File",
    },
    LocaleException {
        key: "*",
        value: "Sponsor",
    },
    LocaleException {
        key: "*",
        value: "Layout",
    },
    LocaleException {
        key: "*",
        value: "Code",
    },
    LocaleException { key: "*", value: "Links" },
    LocaleException { key: "*", value: "Theme" },
    LocaleException { key: "*", value: "Architecture" },
    LocaleException { key: "*", value: "Documentation" },
    LocaleException { key: "*", value: "Text" },
    LocaleException { key: "section_editor", value: "Editor" },
    LocaleException { key: "section_general", value: "General" },
    LocaleException { key: "severity_error", value: "Error" },
    LocaleException { key: "severity_warning", value: "Warning" },
    LocaleException { key: "severity_ignore", value: "Ignore" },
    LocaleException { key: "*", value: "Linter" },
    LocaleException { key: "*", value: "Draw.io / Mermaid" },
];

pub struct LocaleValueOps;

impl LocaleValueOps {
    pub fn compare_locale_values(
        file: &Path,
        en_values: &BTreeMap<String, String>,
        actual_values: &BTreeMap<String, String>,
    ) -> Vec<Violation> {
        en_values
            .iter()
            .filter_map(|(path, en_value)| {
                let actual = actual_values.get(path)?;
                let message = Self::value_violation(path, en_value, actual)?;
                Some(ViolationReporterOps::locale_violation(file, message))
            })
            .collect()
    }

    fn value_violation(path: &str, en_value: &str, actual: &str) -> Option<String> {
        if actual.contains("[TODO") || actual.contains("[todo") {
            return Some(format!(
                "Locale value at `{path}` contains a pseudo-translation placeholder (\"{actual}\"). Lazy cheat detected. Please provide a true native translation."
            ));
        }
        if EMOJI_REGEX.is_match(actual) {
            return Some(format!(
                "Locale value at `{path}` contains an embedded emoji/icon (\"{actual}\"). Embedded emojis/icons are not allowed. Please use SVG components instead (e.g., `{{{{os_cmd:save_document}}}}`: `)."
            ));
        }
        if actual == en_value && !Self::is_allowed_duplicate(path, actual) {
            return Some(format!(
                "Locale value at `{path}` is identical to English baseline (\"{actual}\"). Please translate it."
            ));
        }
        None
    }

    pub fn is_allowed_duplicate(path: &str, value: &str) -> bool {
        if LinterParserOps::is_allowed_string(value) {
            return true;
        }

        LOCALE_VALUE_EXCEPTIONS.iter().any(|ex| {
            let key_matches = ex.key == "*"
                || ex.key == path
                || path.split('.').next_back().is_some_and(|k| k == ex.key);
            key_matches && (ex.value == "*" || ex.value == value)
        })
    }
}
