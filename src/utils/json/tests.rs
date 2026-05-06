use super::*;

#[test]
fn parse_json_file_handles_missing_file() {
    let result = LinterJsonOps::parse_json_file(Path::new("missing_file_random_123.json"));
    let Err(violations) = result else {
        panic!("Expected error for missing file");
    };
    assert!(violations[0].message.contains("Locale file read error"));
}

#[test]
fn parse_json_file_handles_invalid_json() {
    let tmp = tempfile::NamedTempFile::with_suffix(".json").unwrap();
    std::fs::write(tmp.path(), "{invalid json}").unwrap();
    let result = LinterJsonOps::parse_json_file(tmp.path());
    let Err(violations) = result else {
        panic!("Expected error for invalid JSON");
    };
    assert!(violations[0].message.contains("Locale JSON parse error"));
}

#[test]
fn extract_placeholders_handles_edge_cases() {
    let set = LinterJsonOps::extract_placeholders("Hello {unclosed");
    assert!(set.is_empty());

    let set = LinterJsonOps::extract_placeholders("Hello {}");
    assert!(set.is_empty());

    let set = LinterJsonOps::extract_placeholders("Hello {name}");
    assert_eq!(set.len(), 1);
    assert!(set.contains("name"));
}

#[test]
fn collect_json_placeholders_handles_root_array() {
    let value = serde_json::json!(["{item}"]);
    let mut out = BTreeMap::new();
    LinterJsonOps::collect_json_placeholders(&value, None, &mut out);
    assert_eq!(out.get("[0]").unwrap().len(), 1);
}

#[test]
fn collect_json_placeholders_handles_primitives() {
    let value = serde_json::json!(123);
    let mut out = BTreeMap::new();
    LinterJsonOps::collect_json_placeholders(&value, None, &mut out);
    assert!(out.is_empty());
}
