use katana_ast_lint::AstLinterOps;
use katana_ast_lint::rules::{
    FileLengthOps, FunctionLengthOps, LazyCodeOps, NestingDepthOps, ProhibitedAttributesOps,
    TypeSeparationOps,
};
use std::path::PathBuf;

fn source_roots() -> Vec<PathBuf> {
    vec![PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src")]
}

#[test]
fn repository_has_no_lazy_code() {
    AstLinterOps::run(
        "lazy-code",
        "Remove `todo!()`, `unimplemented!()`, and `dbg!()` macros.",
        &source_roots(),
        LazyCodeOps::lint,
    );
}

#[test]
fn repository_has_no_allow_dead_code() {
    AstLinterOps::run(
        "prohibited-attributes",
        "Remove `#[allow(dead_code)]`; delete unused code instead.",
        &source_roots(),
        ProhibitedAttributesOps::lint,
    );
}

#[test]
fn repository_files_stay_within_limit() {
    AstLinterOps::run(
        "file-length",
        "Split files that exceed the 200-line responsibility boundary.",
        &source_roots(),
        FileLengthOps::lint,
    );
}

#[test]
fn repository_functions_stay_focused() {
    AstLinterOps::run(
        "function-length",
        "Extract helper methods when functions exceed the focused line limit.",
        &source_roots(),
        FunctionLengthOps::lint,
    );
}

#[test]
fn repository_nesting_stays_shallow() {
    AstLinterOps::run(
        "nesting-depth",
        "Use early returns or extract helpers instead of deep nesting.",
        &source_roots(),
        NestingDepthOps::lint,
    );
}

#[test]
fn repository_keeps_types_separated() {
    AstLinterOps::run(
        "type-separation",
        "Move public types to dedicated modules when implementation logic grows.",
        &source_roots(),
        TypeSeparationOps::lint,
    );
}
