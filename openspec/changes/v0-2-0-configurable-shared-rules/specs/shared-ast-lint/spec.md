## ADDED Requirements

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
