# Changelog

## v0.5.1

- Align the standard structure rule thresholds with the existing KatanA AST lint baseline.
- Set the catalog defaults for `function-length` and `nesting-depth` to 30 lines and 3 levels.
- Preserve the built-in `file-length` split of 200 lines for source files and 300 lines for Rust test module files.
- Skip redundant post-merge checks after release PRs have already passed preflight.

## v0.5.0

- Introduce `KalRunError` enum to distinguish between violations, configuration errors, and system errors.
- Update `try_from_workspace` and `try_assert_clean` to return `Result<_, KalRunError>` (**breaking change**).
- Add `--json` and `--text` flags to `kal check` for temporary reporter mode overrides.
- Refine CLI exit code mapping: `Violations` → exit 1, `Configuration`/`System` → exit 2.

## v0.4.0

- Introduce `kal` CLI binary for standalone linting.
- Add `kal check` command to execute the standard rule catalog.
- Support stable exit codes (0: clean, 1: violations, 2: errors).
- Refactor library internals to support graceful error handling via `try_*` APIs.
- Share the same rule catalog, `kal.json` interpretation, and reporter backend between API and CLI.

## v0.3.0

- Introduce `KatanaAstLint` one-line repository runner.
- Centralize rule metadata and implementation in a new Rule Catalog.
- Add support for workspace-level `kal.json` discovery.
- Migrate internal engine to a catalog-driven architecture.
- Maintain backward compatibility for `AstLinterOps` and individual rule APIs.

## v0.2.0

- Introduce `kal.json` configuration support.
- Add support for source roots, severities, and thresholds.
- Implement Violation Reporter with JSON and Text modes.

## v0.1.0

- Bootstrap `katana-ast-lint` as a library-only crate.
- Migrate KatanA AST lint rules out of the KatanA workspace.
- Add repository quality gates, CI, release workflow, and OpenSpec planning.
