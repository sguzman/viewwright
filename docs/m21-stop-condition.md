# M21 — Stop Condition

Stop M21 when:

- same-axis fixed size and positive region grow both survive layout allocation;
- horizontal and vertical behavior are covered by deterministic tests;
- fixed-only, grow-only, nested-composition, padding/gap, and canonical layout regressions remain unchanged;
- no new sizing syntax or general layout system has entered scope;
- egui continues consuming the same backend-independent `LayoutPlan`.

Do not continue into overflow policy, shrink behavior, min/max constraints, responsive layout, or M22.