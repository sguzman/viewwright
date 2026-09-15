# M17 Acceptance — Structural Identifier Fidelity

M17 is accepted when all of the following are true:

1. `screen.id` rejects empty and whitespace-only values.
2. Region IDs reject empty and whitespace-only values.
3. Element IDs reject empty and whitespace-only values.
4. Composition IDs reject empty and whitespace-only values.
5. Fixture IDs reject empty and whitespace-only values.
6. Collection item IDs reject empty and whitespace-only values.
7. Tree node IDs reject empty and whitespace-only values.
8. Valid nonblank authored IDs survive resolution unchanged.
9. `screen.root` resolution no longer relies on an authored-empty-compatible string sentinel for success/failure state.
10. Successful `ResolvedBlueprint.root` still exposes the exact authored composition ID.
11. Existing duplicate-ID and missing-reference diagnostics remain effective.
12. Local collection/tree duplicate, selection, parent, and cycle validation remains effective.
13. Canonical accepted sources require no migration.
14. M0–M16 regressions remain passing.
15. No renderer/layout behavior changes.
16. No broad identifier grammar, normalization, newtype migration, or M18 scope is introduced.

Human screenshot QA is not required unless implementation unexpectedly changes rendering behavior.