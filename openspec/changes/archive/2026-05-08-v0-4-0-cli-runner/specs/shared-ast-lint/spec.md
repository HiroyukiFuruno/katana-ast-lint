## ADDED Requirements

### Requirement: AST lint provides a CLI runner

The system SHALL provide a `kal` CLI binary that executes the KAL standard rule set through the shared runner backend.

#### Scenario: Run lint from a command

- **WHEN** a consumer repository runs `kal check`
- **THEN** KAL resolves the workspace root
- **THEN** KAL loads `kal.json` when present
- **THEN** KAL executes all cataloged standard rules
- **THEN** KAL reports violations through the configured reporter contract

### Requirement: CLI runner shares the library backend

The system MUST use the same runner backend for API execution and CLI execution.

#### Scenario: Compare API and CLI results

- **WHEN** the same repository is linted through the one-line API runner and `kal check`
- **THEN** both executions use the same rule catalog
- **THEN** both executions use the same `kal.json` interpretation
- **THEN** both executions produce equivalent violation results

### Requirement: CLI runner uses stable exit codes

The system SHALL expose stable exit codes for CLI consumers.

#### Scenario: Exit after CLI execution

- **WHEN** `kal check` finishes
- **THEN** exit code `0` means no violations
- **THEN** exit code `1` means lint violations were found
- **THEN** exit code `2` means configuration, input, or execution failure

### Requirement: CLI options do not bypass configuration safety

The system MUST NOT provide CLI options that silently disable rules or introduce broad exclusions.

#### Scenario: Request a bypass through CLI

- **WHEN** a consumer needs a scoped allowance
- **THEN** the allowance is expressed through the governed `kal.json` contract
- **THEN** `kal check` does not provide a broad rule-disable or glob-exclude option

## MODIFIED Requirements

### Requirement: AST lint is library-only

The system MAY provide a thin CLI binary after the shared library runner exists, but rule execution and configuration interpretation MUST remain owned by the library backend.

#### Scenario: Package the crate with CLI support

- **WHEN** `katana-ast-lint` is packaged for release
- **THEN** it exposes a Rust library target
- **THEN** it may expose the `kal` binary target
- **THEN** consumers can run lint through tests, repository adapters, CI-backed test commands, or `kal check`
- **THEN** the CLI does not own separate rule behavior
