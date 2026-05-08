# Quality Gates

## Local Targets

| Target | Responsibility | Blocking |
| --- | --- | --- |
| `just fmt-check` | Verify rustfmt output is committed | Yes |
| `just lint` | Run Clippy with zero warnings | Yes |
| `just ast-lint` | Verify repository-specific AST lint invariants | Yes |
| `just test` | Run unit and integration tests | Yes |
| `just openspec-check` | Validate the active OpenSpec change | Yes |
| `just check` | Run all local gates above | Yes |
| `just release-check` | Run local release preflight without requiring crates.io token | Yes |

## AST Lint Invariants

`just ast-lint` protects invariants that normal compiler checks do not cover. Consumer repositories can choose between the one-line runner API or the `kal` CLI.

### Option A: Library API (Recommended for Rust-heavy projects)

Add a test in your integration test suite:

```rust
#[test]
fn repository_ast_lint() {
    katana_ast_lint::KatanaAstLint::from_workspace().assert_clean();
}
```

### Option B: CLI Runner (Recommended for `just` or CI workflows)

Install the CLI and run it from your command line or scripts:

```bash
cargo install katana-ast-lint
kal check
```

For CI stability, it is recommended to pin the version:

```bash
cargo install katana-ast-lint --version 0.4.0 --locked
```

Or run it as a repository-local tool:

```bash
cargo run --package katana-ast-lint --bin kal -- check
```

The standard rule set includes:

- source code must not contain lazy macros such as `todo!`, `unimplemented!`, or `dbg!`
- `#[allow(dead_code)]` must not be used as a cleanup substitute
- production source files must stay within the 200-line responsibility boundary
- function bodies must stay focused
- nesting depth must stay shallow
- public types must stay separated from large implementation files
- i18n and icon consistency rules

## Configuration

Customized behavior belongs in `kal.json` in the workspace root. See `src/config.rs` for the full schema.

```json
{
  "source_roots": ["src"],
  "rules": {
    "file-length": {
      "threshold": 300,
      "severity": "warning"
    }
  }
}
```

## CI Required Checks

The normal CI workflow runs `cargo check`, formatting, Clippy, AST lint, tests,
and OpenSpec validation across macOS, Ubuntu, and Windows. If job names become
required branch-protection checks, update branch protection in the same change
that changes workflow names.

## Release Readiness

Before dispatching a release workflow, run:

```bash
just VERSION=vX.Y.Z release-check
```

The crates.io token is intentionally not required by `release-check`. It is
only required when `publish_crate=true` is used in the release workflow.
