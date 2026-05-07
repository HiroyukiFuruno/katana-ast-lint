# Design: v0.3.0 One-Line Consumer Runner

## Rule Catalog
A central registry of all KAL rules will be implemented. Each entry will include:
- Rule ID
- Category
- Default Severity
- Remediation Guidance
- Implementation Function

## KatanaAstLint Runner
The `KatanaAstLint` struct will provide a fluent API for:
- Loading configuration from the workspace (`kal.json`)
- Filtering and executing rules from the catalog
- Reporting violations according to the configured reporter mode
- Asserting that no violations (or only warnings) exist

## Backward Compatibility
The existing `AstLinterOps::run` and individual rule functions will remain public and functional, but they will internally leverage the catalog metadata where possible.
