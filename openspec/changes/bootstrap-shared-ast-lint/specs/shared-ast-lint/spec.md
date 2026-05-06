## ADDED Requirements

### Requirement: AST lint provides shared repository governance

The system SHALL provide a shared AST lint gate for separated KatanA ecosystem repositories.

#### Scenario: Run lint in a downstream repository

- **WHEN** a downstream repository runs the shared AST lint
- **THEN** it uses the common rule set through a repository adapter
- **THEN** it receives violations in the shared format

### Requirement: AST lint separates common rules from repository adapters

Shared AST lint MUST keep repository-specific file discovery outside common rules.

#### Scenario: Add a KME fixture rule

- **WHEN** KME needs fixture-specific linting
- **THEN** KME provides file discovery and fixture locations through an adapter
- **THEN** the common rule does not hard-code KME or KatanA paths

### Requirement: AST lint rejects exclusion-based bypasses

The system MUST NOT treat lint exclusion as the default fix for AST lint failures.

#### Scenario: A rule cannot be satisfied directly

- **WHEN** a repository cannot satisfy an AST lint rule
- **THEN** the exception reason and alternative design are documented before bypassing
- **THEN** the bypass is not added silently
