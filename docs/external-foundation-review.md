# External Foundation Review

This document evaluates external linting foundations against the requirements of `katana-ast-lint` (KAL) v0.2.0.

## Evaluated Foundations

| Foundation | Strength | Weakness | KAL Relevance |
| :--- | :--- | :--- | :--- |
| **Dylint** | Native Rust linting (Clippy-like), deep type info. | Requires complex toolchain setup, slower for simple AST checks. | High for deep Rust rules, but overkill for structural/naming checks. |
| **ast-grep** | Fast, syntax-aware pattern matching via YAML. | Primarily CLI-driven, integrating as a library in Rust is less standard. | High for structural patterns, but less so for domain-specific logic. |
| **Semgrep** | Multi-language, powerful generic patterns. | Heavyweight, might introduce too many external dependencies. | Low for the current library-only boundary of KAL. |
| **KAL Internal** | Lightweight, library-only, KatanA-domain aware. | Re-implements some AST traversal logic. | Highest. Preserves the library API and KatanA-specific governance. |

## Decision: Maintain KAL as the Rule Boundary

We will continue to use KAL's internal implementation as the primary rule engine for v0.2.0.
