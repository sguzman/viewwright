# M17 Implementation Plan — Structural Identifier Fidelity

1. Read M17 authority plus the current model resolver and ID-validation helpers.
2. Introduce the smallest nonblank-ID validation helper needed for structural identity.
3. Apply it to screen, region, element, composition, fixture, collection-item, and tree-node IDs.
4. Preserve authored valid IDs exactly; validation may inspect `trim()` only to test blankness.
5. Replace the root resolver's empty-string failure sentinel with explicit internal state such as `Option<String>`.
6. Keep successful `ResolvedBlueprint.root` source-faithful.
7. Preserve exact-string reference semantics, duplicate diagnostics, fixture selection/parent validation, tree-cycle validation, and M16 ownership validation.
8. Add compact focused tests for empty/whitespace IDs across the required families plus exact preservation and root-sentinel regression.
9. Verify canonical Project Browser, Reader, visual Reader, Dependency Workbench, and density specimens without source migration.
10. Run `cargo fmt --all`, `cargo test`, `cargo check -p viewwright-preview`, and `git diff --check`.
11. Do not require human visual QA unless rendering/layout code unexpectedly changes.
12. Update README bookkeeping so M16 is recorded as accepted and M17 is recorded as implementation-complete/director-audit-pending after implementation.
13. Comment Issue #18 with implementation evidence, commit/push, leave it open for director acceptance, and stop before M18.