# M35 — Implementation Boundary

M35 is a pure model-comparison milestone.

## In scope

- a small comparator crate/module in ViewWright;
- dependency on `viewwright-expectation`;
- model-only dependency on ViewWitness, default features disabled and pinned to audited revision `f1930ab2a70175c46d12dd1e61501c3b4ae09408`;
- typed deterministic comparison findings;
- exact author-ID indexing/matching;
- exact viewport comparison;
- exact region-bound comparison;
- element parent-author comparison;
- tests using typed ViewWitness witnesses and M33 expectations;
- README milestone bookkeeping.

## Out of scope

- renderer changes;
- AccessKit changes;
- preview capture automation;
- ViewWitness egui/observer features;
- ViewWitness repository mutation;
- expectation wire-format changes;
- authored TOML changes;
- LayoutPlan changes;
- tolerance/fuzzy matching;
- role/importance/overflow/label/action/dominant comparison;
- element geometry;
- screenshot/paint comparison;
- CI gating;
- responsive layout;
- M36.

## Runtime boundary

The comparator consumes already-built data structures. It performs no I/O, capture, rendering, TOML parsing, screenshot processing, or network work.

It must not run on every render frame by default. It is an explicit analysis operation.

## Dependency boundary

Do not enable ViewWitness optional `egui`, `observer`, or `showcase` features. M35 must not introduce a second egui version into ViewWright's active renderer graph.

If model-only ViewWitness cannot be consumed without enabling its egui integration or otherwise contaminates renderer dependencies, STOP and report the dependency conflict instead of working around it by copying the Witness schema.