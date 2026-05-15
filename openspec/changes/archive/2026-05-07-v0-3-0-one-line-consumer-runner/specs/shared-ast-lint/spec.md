## ADDED Requirements

### Requirement: AST lint provides one-line consumer execution

The system SHALL provide a public library runner that lets a consumer repository execute the KAL standard rule set from a one-line test.

#### Scenario: Run the standard rule set from a consumer test

- **WHEN** a consumer repository adds an AST lint integration test
- **THEN** the test can execute the standard KAL rule set with a single public API call
- **THEN** the consumer does not need to manually bind rule ids, remediation guidance, source roots, and lint functions in test code
- **THEN** failures are reported through the configured reporter contract

### Requirement: AST lint runs all standard rules by default

The system SHALL treat the KAL standard rule set as the default execution target for consumer repositories.

#### Scenario: Run without rule selection

- **WHEN** a consumer repository runs the one-line runner
- **THEN** KAL executes all cataloged standard rules
- **THEN** repository-specific differences are loaded from `kal.json`
- **THEN** rules are not silently skipped because the consumer omitted them from runner code

### Requirement: AST lint owns a shared rule catalog

The system SHALL manage rule metadata in a KAL-owned catalog.

#### Scenario: Report a cataloged rule violation

- **WHEN** a standard rule emits a violation
- **THEN** the violation uses the cataloged rule id, severity, and remediation guidance
- **THEN** the consumer repository does not duplicate that metadata in its runner

### Requirement: One-line runner preserves library-only boundaries

The system MUST keep the one-line runner inside the Rust library API.

#### Scenario: Package the crate with the runner

- **WHEN** `katana-ast-lint` is packaged after adding the one-line runner
- **THEN** it still exposes no CLI or binary target
- **THEN** consumers invoke the runner from tests, repository adapters, or CI-backed test commands

## MODIFIED Requirements

### Requirement: AST lint separates common rules from repository adapters

Shared AST lint MUST keep repository-specific file discovery outside common rules while allowing the shared runner to load repository differences from `kal.json`.

#### Scenario: Add a KMM fixture rule

- **WHEN** KMM needs fixture-specific linting
- **THEN** KMM provides file discovery and fixture locations through `kal.json` or a repository adapter
- **THEN** the common rule does not hard-code KMM or KatanA paths
- **THEN** the consumer runner does not duplicate standard rule wiring
