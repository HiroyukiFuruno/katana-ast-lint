# Release Runbook

## Purpose

This runbook covers release checks for the library-only `katana-ast-lint`
crate.

## Preflight Checklist

Confirm `Cargo.toml` metadata is still correct:

- `license = "MIT"`
- `readme = "README.md"`
- `repository` points at the GitHub repository
- `description`, `keywords`, and `categories` are still accurate
- no `[[bin]]` target is present

Run local validation:

```bash
just VERSION=vX.Y.Z release-check
```

## CI/CD Release Flow

The Release workflow is defined in `.github/workflows/release.yml`.

Required sequence:

- Confirm `Cargo.toml` `package.version` is the intended version.
- Confirm `CHANGELOG.md` has a `## vX.Y.Z` section.
- Run `just VERSION=vX.Y.Z release-check`.
- Create a signed annotated tag with `just VERSION=vX.Y.Z release-tag`.
- Dispatch GitHub Release publication with `just VERSION=vX.Y.Z release-github`.
- After the user registers `CARGO_REGISTRY_TOKEN`, dispatch crates.io publication with `just VERSION=vX.Y.Z release-publish`.

The workflow validates:

- Cargo version equals release version.
- release tag is an annotated signed tag that GitHub reports as Verified.
- `just release-check`
- package artifact creation
- GitHub Release creation or update
- optional crates.io publication when `publish_crate=true`

## Required Secrets

- `CARGO_REGISTRY_TOKEN`: crates.io API token used only when release dispatch sets `publish_crate=true`.

The token is a publication prerequisite, not a blocker for code migration,
OpenSpec readiness, or GitHub Release-only preparation.

