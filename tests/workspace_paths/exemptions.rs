use super::Workspace;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

fn write_source(root: &Path, name: &str, statements: &str) -> PathBuf {
    let path = root.join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(&path, format!("fn production() {{\n{statements}\n}}\n")).unwrap();
    path
}

#[test]
fn workspace_ancestors_do_not_activate_rule_exemptions() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join(
        "theme_bridge/widgets/align_center/ui.rs/widgets/menu_button/mod.rs/icon/mod.rs/system/process.rs/project",
    );
    fs::create_dir_all(&root).unwrap();
    let workspace = Workspace::new(&root);
    let sources = workspace.member.join("src");
    let cases = [
        (
            "theme_bridge.rs",
            "ui.painter().rect_stroke(rect, stroke);",
            "before `rect_stroke`",
        ),
        (
            "widgets/align_center/ui.rs",
            "ui.horizontal(|_| {});",
            "Use `AlignCenter`",
        ),
        (
            "widgets/menu_button/mod.rs",
            "ui.menu_button(label, |_| {});",
            "menu_button shows a frame",
        ),
        (
            "icon/mod.rs",
            "ui.add(egui::Button::image(icon));",
            "Button::image",
        ),
        (
            "system/process.rs",
            "std::process::Command::new(command);",
            "Use of `Command::new` detected",
        ),
    ];
    for (path, statement, _) in cases {
        write_source(&sources, path, statement);
    }
    let statements = cases.map(|(_, statement, _)| statement).join("\n");
    let production = write_source(&sources, "probe.rs", &statements);
    let relative_allowance = write_source(&sources, "relative.rs", cases[0].1);
    workspace.write_config(
        "kal.json",
        json!({
            "source_roots": ["crates/member/src"],
            "rules": {
                "frame-stroke": {
                    "allow": [
                        {"path": "project/crates/member/src/probe.rs", "reason": "Outside scope"},
                        {"path": relative_allowance.strip_prefix(&workspace.root).unwrap(), "reason": "Relative"}
                    ]
                }
            }
        }),
    );

    let violations = workspace.diagnostics(None);
    assert_eq!(violations.len(), cases.len());
    for violation in &violations {
        assert_eq!(violation.file.canonicalize().unwrap(), production);
    }
    for (_, _, message) in cases {
        assert!(
            violations
                .iter()
                .any(|violation| violation.message.contains(message)),
            "missing {message}: {violations:#?}"
        );
    }
}
