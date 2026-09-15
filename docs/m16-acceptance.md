# M16 — Acceptance

M16 is accepted when ViewWright enforces the composition-tree ownership invariant already established by M1.

## Required behavior

- duplicate child references inside one composition are rejected;
- the same region referenced by two different compositions is rejected;
- the same non-root composition referenced by two different compositions is rejected;
- the root composition referenced as a child anywhere is rejected;
- existing cycle diagnostics remain intact;
- canonical accepted screens continue resolving unchanged;
- M4 layout geometry remains unchanged for valid trees;
- egui, concept, ASCII, fixtures, actions, density, visual semantics, and M10–M15 behavior remain unchanged.

## Scope

M16 does not require every declared region/composition to be reachable from the root. It only makes placement ownership unambiguous for authored child relationships.

Human visual QA is not inherently required because valid canonical trees and renderer/layout behavior are intended to remain unchanged.