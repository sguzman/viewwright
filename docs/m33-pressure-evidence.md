# M33 — Pressure Evidence

The pressure is architectural and cross-project rather than visual.

ViewWright's architecture has always reserved an `expectation export` branch beside ASCII and egui projection. The repository now has enough resolved semantic and geometry fidelity for that export to carry useful truth rather than placeholders:

- typed screen/region/element identity;
- typed roles, importance, kinds, actions, dominant targets, and overflow;
- deterministic backend-independent LayoutPlan geometry;
- root-reachability guarantees from M29/M30;
- overlay geometry from M31;
- region overflow intent from M32.

ViewWitness independently establishes the complementary epistemic boundary:

- a basic `Witness` is observed testimony;
- source/specification claims are `Intended` and are not part of a basic witness;
- `author_id` is explicit additional identity evidence rather than an observed node ID replacement.

Therefore the earned next step is not to fake a witness. It is to materialize ViewWright's intended side as its own deterministic document.

Pressure cases:

1. `examples/project-browser.toml` at 1440 × 900 proves ordinary linear region geometry, semantic identity, dominant region, and element metadata.
2. `specimens/overlay-command-palette-pressure.toml` at 1440 × 900 proves that overlapping base/floating region geometry survives export without being flattened into fake linear semantics.
3. `specimens/reader-overflow-pressure.toml` at 1440 × 900 proves that `scroll_y` remains an intended region property while runtime scroll offset stays absent.

No current accepted source needs migration.
