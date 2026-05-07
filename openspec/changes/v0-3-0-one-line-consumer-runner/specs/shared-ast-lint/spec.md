# Spec: Shared AST Lint (v0.3.0)

## ADDED Requirements

### Requirement: One-line Repository Runner
The `KatanaAstLint` API MUST provide a way to run all standard rules with a single call from a consumer workspace.

#### Scenario: Running from workspace
Given a repository with a `kal.json` file
When `KatanaAstLint::from_workspace().assert_clean()` is called
Then all enabled rules in the catalog MUST be executed against the configured source roots.

### Requirement: Rule Catalog
The rule catalog MUST be the single source of truth for standard rule metadata.

#### Scenario: Rule registration
Given a new standard rule is implemented
When it is added to the static `RULE_CATALOG`
Then it MUST be automatically included in the one-line runner execution.
