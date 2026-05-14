## ADDED Requirements

### Requirement: Runner returns structured error categories

The system SHALL expose a structured error type that distinguishes lint violations, configuration errors, and system errors from the runner API.

#### Scenario: Receive a violation result

- **WHEN** the runner detects rule violations
- **THEN** it returns an error variant tagged as `Violations`
- **THEN** the variant carries a human-readable message equivalent to the v0.4.0 `String` error

#### Scenario: Receive a configuration error

- **WHEN** the runner cannot parse `kal.json` or resolve the workspace
- **THEN** it returns an error variant tagged as `Configuration`

#### Scenario: Receive a system error

- **WHEN** the runner fails for non-violation, non-configuration reasons
- **THEN** it returns an error variant tagged as `System`

### Requirement: CLI maps runner error categories to stable exit codes

The system MUST map runner error categories to the stable CLI exit codes defined in v0.4.0.

#### Scenario: Lint violations end the run

- **WHEN** the runner returns a `Violations` error
- **THEN** `kal check` exits with code `1`

#### Scenario: Configuration or system errors end the run

- **WHEN** the runner returns a `Configuration` or `System` error
- **THEN** `kal check` exits with code `2`

### Requirement: CLI provides a reporter mode override

The system SHALL allow CLI consumers to override the reporter output mode for a single invocation through `--json` or `--text`.

#### Scenario: Force JSON output

- **WHEN** a consumer runs `kal check --json`
- **THEN** the reporter emits JSON regardless of the `kal.json` `reporter.mode`
- **THEN** the override does not modify any persisted configuration

#### Scenario: Force text output

- **WHEN** a consumer runs `kal check --text`
- **THEN** the reporter emits text regardless of the `kal.json` `reporter.mode`

#### Scenario: Reject conflicting overrides

- **WHEN** a consumer passes both `--json` and `--text`
- **THEN** `kal check` reports a usage error and exits with code `2`

### Requirement: Reporter mode override does not bypass configuration safety

The system MUST limit CLI overrides to the reporter output mode and MUST NOT introduce CLI options that disable rules or expand exclusions.

#### Scenario: Attempt to disable rules through CLI

- **WHEN** a consumer wants to disable a rule for a single CLI run
- **THEN** `kal check` does not provide a rule-disable option
- **THEN** the consumer expresses the change through the governed `kal.json` contract

## MODIFIED Requirements

### Requirement: CLI runner uses stable exit codes

The system SHALL expose stable exit codes for CLI consumers, mapped from the runner's structured error categories.

#### Scenario: Exit after CLI execution

- **WHEN** `kal check` finishes
- **THEN** exit code `0` means no violations
- **THEN** exit code `1` means the runner returned a `Violations` error
- **THEN** exit code `2` means the runner returned a `Configuration` or `System` error, or the CLI received invalid arguments

### Requirement: CLI runner shares the library backend

The system MUST use the same runner backend for API execution and CLI execution, including the structured error type.

#### Scenario: Compare API and CLI results

- **WHEN** the same repository is linted through the one-line API runner and `kal check`
- **THEN** both executions use the same rule catalog
- **THEN** both executions use the same `kal.json` interpretation
- **THEN** both executions surface the same error category for equivalent failures
- **THEN** both executions produce equivalent violation results
