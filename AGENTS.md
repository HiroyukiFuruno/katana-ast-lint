# Project Rules

- This repository is library-only. Do not add a CLI or binary target unless an OpenSpec change explicitly changes that boundary.
- `README.md`, `CHANGELOG.md`, and public Markdown under `docs/` must be written in English.
- The repository default branch is `master`. Documentation-only, OpenSpec-only, and workflow-maintenance changes that do not affect the library body should be done directly on `master`.
- CI/CD and release dispatch settings must target `master`; do not leave `main` as a default or release reference.
- Use `scripts/openspec` instead of calling a bare `openspec` command from agent instructions or local workflow docs.
- Run `just check` before commit and release handoff.
- Do not bypass `lefthook`, Clippy, AST lint, or OpenSpec validation.
