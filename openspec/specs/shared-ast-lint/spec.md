# shared-ast-lint Specification

## Purpose
katanaシリーズのrepository間で共有するRust AST lintの公開契約を定義する。

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

### Requirement: AST lint supports repository configuration

The system SHALL support root-level `kal.json` for repository-specific shared rule configuration.

#### Scenario: Configure repository source roots

- **WHEN** a consumer repository runs KAL through its test or CI runner
- **THEN** it can provide source roots through `kal.json`
- **THEN** shared rules do not hard-code repository-local paths

#### Scenario: Keep v0.1.0 compatibility

- **WHEN** `kal.json` is missing
- **THEN** KAL uses v0.1.0-compatible defaults
- **THEN** existing consumers can adopt configuration incrementally

### Requirement: Configuration does not become a bypass

The system MUST NOT treat `kal.json` as a broad exclusion or rule-disabling mechanism.

#### Scenario: Add a scoped allowance

- **WHEN** a repository needs a scoped allowance
- **THEN** the allowance includes the target rule, reason, and review condition
- **THEN** the allowance does not disable the rule globally

### Requirement: AST lint evaluates external lint foundations

The system SHALL evaluate external lint foundations before expanding KAL's internal rule engine.

#### Scenario: Decide whether to use an external library

- **WHEN** v0.2.0 implementation begins
- **THEN** Dylint, ast-grep, Semgrep, and the current KAL internal implementation are compared
- **THEN** parsing or rule execution responsibilities are delegated only when the external library improves robustness without breaking the library-only consumer API
- **THEN** KAL remains the shared KatanA ecosystem rule boundary

### Requirement: AST lint reports machine-readable violations

The system SHALL define severity and JSON output for shared AST lint violations.

#### Scenario: CI consumes JSON output

- **WHEN** a consumer repository requests JSON output
- **THEN** violations include rule id, severity, file, location, message, and remediation guidance
- **THEN** text output remains available for local human review
