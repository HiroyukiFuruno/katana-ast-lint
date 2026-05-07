use katana_ast_lint::KatanaAstLint;

#[test]
fn repository_ast_lint() {
    KatanaAstLint::from_workspace().assert_clean();
}
