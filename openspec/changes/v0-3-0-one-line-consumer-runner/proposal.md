# Proposal: v0.3.0 One-Line Consumer Runner

## Problem
Consumer repositories currently need to manually bundle and execute each AST lint rule in their test suites. This leads to boilerplate and makes it difficult to propagate new standard rules across the KatanA ecosystem.

## Solution
Introduce a centralized rule catalog and a high-level runner API (`KatanaAstLint`) that allows running all standard rules with a single line of code, while still respecting repository-local `kal.json` configurations.

## Proposed API
```rust
katana_ast_lint::KatanaAstLint::from_workspace().assert_clean();
```
