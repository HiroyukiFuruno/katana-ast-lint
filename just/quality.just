# Apply rustfmt
fmt:
    cargo fmt --all

# Check rustfmt output
fmt-check:
    cargo fmt --all -- --check

# Run Clippy with zero warnings
lint:
    cargo clippy -j {{JOBS}} --workspace --all-targets --all-features --locked -- -D warnings

# Run repository AST lint
ast-lint:
    cargo test -j {{JOBS}} --test repository_ast_lint --locked -- --nocapture

# Run unit and integration tests
test:
    cargo test --workspace --locked

# Run OpenSpec validation for active changes
openspec-check:
    scripts/openspec validate "bootstrap-shared-ast-lint" --strict

# Local quality gate
check: fmt-check lint ast-lint test openspec-check
    @echo "All checks passed"

