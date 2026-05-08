# Project Rules

- This repository provides a thin CLI (`kal`) in addition to the library API. Rule implementation and configuration interpretation must remain in the library.
- `README.md`, `CHANGELOG.md`, and public Markdown under `docs/` must be written in English.
- The repository default branch is `master`. Documentation-only, OpenSpec-only, and workflow-maintenance changes that do not affect the library body should be done directly on `master`.
- CI/CD and release dispatch settings must target `master`; do not leave `main` as a default or release reference.
- Use `scripts/openspec` instead of calling a bare `openspec` command from agent instructions or local workflow docs.
- Run `just check` before commit and release handoff.
- Do not bypass `lefthook`, Clippy, AST lint, or OpenSpec validation.
- Before clearing local diffs or stashes, inspect their contents and decide whether each item should be incorporated, ignored, or kept for user review.
