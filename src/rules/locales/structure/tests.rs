use super::*;

#[test]
fn compare_locale_shape_detects_missing_keys() {
    let expected = BTreeMap::from([
        ("menu".to_string(), JsonNodeKind::Object),
        ("menu.file".to_string(), JsonNodeKind::String),
    ]);
    let actual = BTreeMap::from([("menu".to_string(), JsonNodeKind::Object)]);
    let violations =
        LocaleStructureOps::compare_locale_shape(Path::new("locale.json"), &expected, &actual);
    assert_eq!(violations.len(), 1);
    assert!(violations[0].message.contains("menu.file"));
}

#[test]
fn compare_locale_placeholders_detects_mismatch() {
    let expected_shape = BTreeMap::from([("status.save_failed".to_string(), JsonNodeKind::String)]);
    let expected_placeholders = BTreeMap::from([(
        "status.save_failed".to_string(),
        BTreeSet::from(["error".to_string()]),
    )]);
    let actual_placeholders = BTreeMap::from([(
        "status.save_failed".to_string(),
        BTreeSet::from(["message".to_string()]),
    )]);

    let violations = LocaleStructureOps::compare_locale_placeholders(
        Path::new("locale.json"),
        &expected_shape,
        &expected_placeholders,
        &actual_placeholders,
    );
    assert_eq!(violations.len(), 1);
    assert!(violations[0].message.contains("status.save_failed"));
}

#[test]
fn build_locale_baseline_returns_errors_for_mismatched_bases() {
    let tmp = tempfile::TempDir::new().unwrap();
    let ja_path = tmp.path().join("ja.json");
    let en_path = tmp.path().join("en.json");
    std::fs::write(
        &ja_path,
        r#"{"status":{"saved":"saved","failed":"failed: {error}"}}"#,
    )
    .unwrap();
    std::fs::write(
        &en_path,
        r#"{"status":{"saved":"Saved.","failed":"Failed: {message}"}}"#,
    )
    .unwrap();

    let violations = LocaleStructureOps::build_locale_baseline(&ja_path, &en_path)
        .expect_err("base locales should mismatch");
    assert!(!violations.is_empty());
    assert!(
        violations
            .iter()
            .any(|v| v.message.contains("Locale placeholder mismatch"))
    );
}
