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
- Merge a release PR from `release/vX.Y.Z` into `master`; the workflow creates
  the signed annotated tag, GitHub Release, and crates.io publication
  automatically.
- For one-off publication, create a signed annotated tag with
  `just VERSION=vX.Y.Z release-tag`, then dispatch GitHub Release publication
  with `just VERSION=vX.Y.Z release-github`.
- After the user registers `CARGO_REGISTRY_TOKEN`, dispatch crates.io publication with `just VERSION=vX.Y.Z release-publish`.

The workflow validates:

- Cargo version equals release version.
- release PR merge events come from a repository-owned `release/vX.Y.Z` branch.
- release tag is an annotated signed tag that GitHub reports as Verified.
- `just release-check`
- package artifact creation
- GitHub Release creation or update
- crates.io publication for merged `release/vX.Y.Z` PRs
- optional crates.io publication for manual dispatch when `publish_crate=true`

## Required Secrets

- `RELEASE_GPG_KEY`: armored private key used only when a merged `release/vX.Y.Z`
  PR needs the workflow to create the signed tag. Register it as a repository
  secret, not as a visible repository variable.
- `RELEASE_GPG_PASSPHRASE`: passphrase for `RELEASE_GPG_KEY` when that key is
  protected.
- `CARGO_REGISTRY_TOKEN`: crates.io API token used by merged `release/vX.Y.Z`
  PRs and manual dispatch when `publish_crate=true`.

The token is a publication prerequisite, not a blocker for code migration,
OpenSpec readiness, or GitHub Release-only preparation.
