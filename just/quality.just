# Apply rustfmt
fmt:
    {{CARGO}} fmt --all

# Check rustfmt output
fmt-check:
    {{CARGO}} fmt --all -- --check

# Run Clippy with zero warnings
lint:
    {{CARGO}} clippy -j {{JOBS}} --workspace --all-targets --all-features --locked -- -D warnings

# Run repository AST lint
ast-lint:
    {{CARGO}} test -j {{JOBS}} --test repository_ast_lint --locked -- --nocapture

# Run unit and integration tests
test:
    {{CARGO}} test --workspace --locked

# Run OpenSpec validation for active changes
openspec-check:
    scripts/openspec validate --changes --strict

# Local quality gate
check: fmt-check lint ast-lint test openspec-check
    @echo "All checks passed"
