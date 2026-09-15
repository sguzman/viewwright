# M18 Acceptance — Global Composition Cycle Fidelity

M18 is accepted when all of the following hold:

- an ordinary root-reachable composition cycle still fails;
- an unreachable authored composition cycle also fails;
- an unreachable self-cycle fails;
- unused acyclic compositions remain valid;
- unused regions remain valid;
- cycle diagnostics are deterministic and useful;
- M16 duplicate-sibling, multiple-parent, root-as-child, and unused-declaration behavior remains intact;
- canonical accepted sources require no migration;
- layout, egui, ASCII, concept, and semantic output for valid sources are unchanged;
- no new TOML syntax or general graph abstraction is introduced.

Focused tests should include a root-reachable cycle, an unreachable multi-node cycle, an unreachable self-cycle, and a valid unused acyclic composition.

M18 has no intended visible delta, so human screenshot QA is unnecessary unless implementation unexpectedly changes rendering/layout code.
