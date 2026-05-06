# katana-ast-lint

Reusable AST lint rules for KatanA ecosystem repositories.

`katana-ast-lint` is a library-only crate. It does not provide a CLI. Consumer
repositories call the public Rust API from their own tests or quality gates.

## Scope

- Shared Rust AST lint rules migrated from the KatanA workspace
- Repository adapters for consumer-specific paths and fixtures
- Structured violations with file, line, column, and message
- Quality gates for separated repositories such as KME, preview, editor, export, and shared widgets

## Library Usage

```rust
use katana_ast_lint::rules::LazyCodeOps;
use katana_ast_lint::utils::LinterParserOps;
use std::path::Path;

let path = Path::new("src/lib.rs");
let syntax = LinterParserOps::parse_file(path)?;
let violations = LazyCodeOps::lint(path, &syntax);
```

## Local Development

```bash
just check
```

`just check` runs formatting, Clippy, repository AST lint tests, and unit tests.
