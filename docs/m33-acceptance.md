# M33 — Acceptance

M33 is accepted when all of the following hold:

- a typed ViewWright expectation model exists;
- expectation serialization is explicitly epistemic `intended` rather than observed;
- the document carries a format version, screen author ID, and explicit logical viewport size;
- typed dominant target identity is preserved when present;
- only root-reachable regions/elements are exported;
- unused declarations remain legal but are absent from exported expectations;
- region expectations preserve author ID, role, importance, LayoutPlan bounds, and overflow policy;
- element expectations preserve author ID, owning region author ID, kind, importance, exact label, and command action when present;
- element geometry is not invented;
- output is deterministic for identical resolved input + viewport;
- region bounds exactly match LayoutPlan at the requested viewport;
- overlay floating/base region bounds export honestly;
- scroll_y intent exports as intent without runtime scroll position;
- existing canonical blueprints require no migration;
- the exporter does not depend on, mutate, or serialize as a ViewWitness `Witness`;
- no comparison, tolerance, observation, fixture-content, or responsive system is introduced;
- M0–M32 regressions remain green.

No human screenshot QA is required if implementation has no intended visible UI delta and tests/goldens prove the exported artifacts.
