# shared-ast-lint Specification

## Purpose
KatanA ecosystem repositories share AST lint rules, violation contracts, and quality gates through a library-only crate.

## Requirements
### Requirement: AST lint provides shared repository governance

The system SHALL provide a shared AST lint gate for separated KatanA ecosystem repositories.

#### Scenario: Run lint in a downstream repository

- **WHEN** a downstream repository runs the shared AST lint
- **THEN** it uses the common rule set through a repository adapter or test runner
- **THEN** it receives violations in the shared v0 location format

### Requirement: AST lint is library-only

The system MUST NOT provide a CLI for v0.

#### Scenario: Package the crate

- **WHEN** `katana-ast-lint` is packaged for release
- **THEN** it exposes a Rust library target
- **THEN** it does not expose a binary target
- **THEN** consumers run lint through their own tests or CI jobs

### Requirement: AST lint separates common rules from repository adapters

Shared AST lint MUST keep repository-specific file discovery outside common rules.

#### Scenario: Add a KME fixture rule

- **WHEN** KME needs fixture-specific linting
- **THEN** KME provides file discovery and fixture locations through an adapter
- **THEN** the common rule does not hard-code KME or KatanA paths

### Requirement: AST lint follows kml-style quality gates

The system SHALL provide local and CI quality gates aligned with the katana-markdown-linter repository shape.

#### Scenario: Run repository quality gate

- **WHEN** a developer runs `just check`
- **THEN** formatting, Clippy, repository AST lint, tests, and OpenSpec validation run
- **THEN** failures block release handoff

### Requirement: Crates.io token is not required for migration

The system SHALL treat `CARGO_REGISTRY_TOKEN` registration as a user-side release prerequisite.

#### Scenario: Prepare release before token registration

- **WHEN** migration and release preparation are completed
- **THEN** local release checks and GitHub Release preparation can be validated
- **THEN** crates.io publication waits until the user registers `CARGO_REGISTRY_TOKEN`

### Requirement: AST lint rejects exclusion-based bypasses

The system MUST NOT treat lint exclusion as the default fix for AST lint failures.

#### Scenario: A rule cannot be satisfied directly

- **WHEN** a repository cannot satisfy an AST lint rule
- **THEN** the exception reason and alternative design are documented before bypassing
- **THEN** the bypass is not added silently

### Requirement: AST lint defines the v0.2.0 repository configuration plan

The system SHALL define `kal.json` as the planned v0.2.0 repository configuration file for shared AST lint without requiring v0.1.0 to implement config loading.

#### Scenario: Plan repository-specific lint inputs

- **WHEN** the v0.2.0 OpenSpec is authored
- **THEN** it defines root-level `kal.json` as the place for repository-specific target directories, thresholds, allow lists, and domain rule input paths
- **THEN** it requires shared rules to avoid repository-specific hard-coded paths
- **THEN** it requires missing `kal.json` to keep v0.1.0-compatible defaults

#### Scenario: Plan bypass prevention

- **WHEN** the v0.2.0 OpenSpec defines scoped allowances
- **THEN** it does not define `kal.json` as an undocumented broad exclusion list
- **THEN** it requires the exception reason and alternative design to be documented before any scoped allowance is added

#### Scenario: Evaluate external lint foundations

- **WHEN** the v0.2.0 OpenSpec is authored
- **THEN** it evaluates whether Dylint, ast-grep, Semgrep, or another external library should own any parsing or rule execution responsibility
- **THEN** KAL remains scoped to shared KatanA ecosystem rules rather than becoming a general-purpose Rust lint framework
