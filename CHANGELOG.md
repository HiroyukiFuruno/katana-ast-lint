# Changelog

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
