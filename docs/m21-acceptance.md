# M21 — Acceptance

M21 is accepted when ViewWright preserves both authored fixed main-axis size and authored growth weight for the same region.

## Required behavior

- horizontal region `width + grow` resolves unchanged and lays out as fixed base plus proportional remaining-width share;
- vertical region `height + grow` resolves unchanged and lays out as fixed base plus proportional remaining-height share;
- fixed-only regions retain existing geometry;
- grow-only regions retain existing geometry;
- cross-axis width/height does not suppress main-axis growth;
- nested composition growth remains unchanged;
- canonical accepted specimens retain their existing layout geometry;
- no new source syntax is introduced.

## Validation

M21 does not make fixed+grow invalid. Both fields are already legal source semantics. Existing non-negative finite grow validation remains authoritative.

## Tests

Add deterministic layout tests covering at least:

- horizontal fixed+grow versus a grow-only sibling;
- vertical fixed+grow versus a grow-only sibling;
- proportional distribution when more than one fixed+grow/grow child participates;
- fixed-only and grow-only regression behavior;
- canonical Reader, Dependency Workbench, Project Browser, and density layout regressions.

## Runtime/visual QA

No canonical visible delta is intended. Human screenshot QA is unnecessary if canonical layout regression tests remain identical and egui consumes the same layout-plan API.