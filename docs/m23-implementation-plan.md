# M23 — Implementation Plan

1. Add source-oriented blankness validation for every collection-item `label` and tree-node `label` during fixture resolution.
2. Use `trim().is_empty()` only for validity; copy valid labels unchanged into resolved items/nodes.
3. Keep IDs authoritative for selection and tree parent relationships; duplicate labels remain legal.
4. Add tests for empty/whitespace rejection, exact preservation, duplicate-label acceptance, missing-label parse behavior, and existing fixture regressions.
5. Verify canonical Project Browser, Reader variants, Dependency Workbench, and density specimens need no migration.
6. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
7. Update README to M22 accepted and M23 implementation complete/director audit pending.
8. Commit/push and comment Issue #24, leaving it open for director audit.