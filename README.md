<p align="center">
  <img src="assets/kal-icon.png" width="128" alt="katana-ast-lint icon">
</p>

<h1 align="center">katana-ast-lint</h1>

<p align="center">
  A shared Rust AST lint crate for KatanA ecosystem governance.
</p>

<p align="center">
  <strong><a href="#installation">Installation</a></strong> |
  <strong><a href="#cli-usage">CLI Usage</a></strong> |
  <strong><a href="#library-api">Library API</a></strong> |
  <strong><a href="#downstream-integration">Downstream Integration</a></strong> |
  <strong><a href="docs/quality-gates.md">Quality Gates</a></strong> |
  <strong><a href="docs/release-runbook.md">Release</a></strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT"></a>
  <a href="https://github.com/HiroyukiFuruno/katana-ast-lint/actions/workflows/test-and-build.yml"><img src="https://github.com/HiroyukiFuruno/katana-ast-lint/actions/workflows/test-and-build.yml/badge.svg" alt="CI"></a>
  <a href="https://github.com/HiroyukiFuruno/katana-ast-lint/releases/latest"><img src="https://img.shields.io/github/v/release/HiroyukiFuruno/katana-ast-lint" alt="Latest Release"></a>
  <a href="https://crates.io/crates/katana-ast-lint"><img src="https://img.shields.io/crates/v/katana-ast-lint.svg" alt="crates.io"></a>
  <a href="https://docs.rs/katana-ast-lint"><img src="https://img.shields.io/badge/docs.rs-katana--ast--lint-blue" alt="docs.rs"></a>
</p>

---

## What is katana-ast-lint

`katana-ast-lint` provides reusable Rust syntax-tree checks for KatanA ecosystem
repositories. It was extracted from the KatanA workspace so separated crates can
share the same structural rules, violation shape, and release-grade quality
gate.

KAL provides both a library API and a thin CLI (`kal`). Consumer repositories
can choose the integration method that best fits their workflow—whether it's
running as a Rust test or as a standalone command in CI.

KAL is the shared rule boundary for the KatanA series, not a replacement for
Rust linting foundations such as Clippy, Dylint, ast-grep, or Semgrep. Future
configuration work can delegate parsing or rule execution to mature external
libraries when that makes the shared rules more robust.

## Features

- **One-line Repository Runner** to execute all standard rules with a single call.
- **Thin CLI (`kal`)** for easy integration into `just` or CI workflows.
- **Shared Rust AST rules** migrated from the KatanA workspace.
- **Structured violations** with file, line, column, and message fields.
- **Adapter-friendly API** so repository-specific paths and fixtures stay out of
  common rules.
- **KatanA-compatible defaults** for file length, function length, and nesting
  depth.
- **Repository quality gates** for KMM, preview, editor, export, widget, and
  KatanA integration work.

## Installation

To use the library API, add the crate to your `dev-dependencies`:

~~~bash
cargo add katana-ast-lint --dev
~~~

To use the CLI, install it via `cargo`:

~~~bash
cargo install katana-ast-lint
~~~

Use a path dependency while developing sibling repositories locally:

~~~toml
[dev-dependencies]
katana-ast-lint = { path = "../katana-ast-lint" }
~~~

## CLI Usage

Run the standard rule set from the command line:

~~~bash
kal check
~~~

Force a specific output mode for a single run:

~~~bash
kal check --json
kal check --text
~~~

The CLI resolves the workspace root, loads `kal.json` if present, and executes
all standard rules.

## Library API

Run a repository gate from an integration test using the recommended one-line runner:

~~~rust
#[test]
fn repository_ast_lint() {
    katana_ast_lint::KatanaAstLint::from_workspace().assert_clean();
}
~~~

Alternatively, run an individual rule:

~~~rust
use katana_ast_lint::rules::LazyCodeOps;
use katana_ast_lint::utils::LinterParserOps;
use std::path::Path;

let path = Path::new("src/lib.rs");
let syntax = LinterParserOps::parse_file(path)?;
let violations = LazyCodeOps::lint(path, &syntax);
~~~

## Downstream Integration

Consumer repositories should expose their own `just ast-lint` target and call
`katana_ast_lint` from tests or a repository-local adapter. This keeps each
repository's source roots, fixtures, and scoped allowances local while preserving
the shared rule implementation.

Recommended shape:

~~~text
crates/<repo>-linter/
  Cargo.toml
  tests/ast_linter.rs
kal.json
just/quality.just
~~~

The `kal.json` file in the repository root allows you to configure source roots,
rule severities, optional rule enablement, and repository-specific inputs. Keep
standard thresholds out of downstream config unless the repository intentionally
deviates from KAL defaults.

~~~json
{
  "source_roots": ["src"],
  "rules": {
    "i18n": { "enabled": true },
    "locales": {
      "enabled": true,
      "inputs": ["locales"]
    }
  }
}
~~~

## Quality Gates

Run the local gate before committing:

~~~bash
just check
~~~

`just check` runs formatting, Clippy, repository AST lint, unit tests, and
OpenSpec validation. Use focused targets when iterating:

~~~bash
just fmt-check
just lint
just ast-lint
just test
just openspec-check
~~~

## Release Policy

`Cargo.toml` is the version source of truth. Release branches should use the
same `release/vX.Y.Z` shape as sibling KatanA ecosystem repositories when a
release PR is needed.

- Run `just VERSION=vX.Y.Z release-check` before publication.
- Merge `release/vX.Y.Z` into `master` to create the signed tag, GitHub Release,
  and crates.io publication automatically.
- Run `just VERSION=vX.Y.Z release-github` to create or update only the GitHub
  Release manually.
- Run `just VERSION=vX.Y.Z release` when crates.io publication is intended.
- GitHub Releases require a signed annotated `vX.Y.Z` tag that GitHub reports as
  Verified.
- `just release` stops before dispatch when the requested version already exists
  on crates.io.
- crates.io publication requires the `CARGO_REGISTRY_TOKEN` GitHub secret.

See [`docs/release-runbook.md`](docs/release-runbook.md) for the full release
sequence and [`docs/quality-gates.md`](docs/quality-gates.md) for required gates.

## Non-Goals

- Replacing `rustc`, rustfmt, or Clippy.
- Providing complex CLI options that bypass `kal.json`.
- Embedding KMM, preview, editor, export, widget, or KatanA application types.
- Using broad exclusions as the default fix for rule failures.

## License

MIT - see [LICENSE](LICENSE).
