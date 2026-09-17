# M37 — Canonical Audit

M37 is constrained by M31–M36 and by the pinned ViewWitness revision already audited in M35.

## Current ViewWright toolkit surface

The workspace currently centralizes:

```text
egui = 0.31
eframe = 0.31
```

`viewwright-egui` enables egui's AccessKit feature. `viewwright-preview` consumes workspace eframe.

The renderer uses ordinary egui layout/rendering plus M34 non-visual AccessKit anchors.

## Current ViewWitness boundary

M35 pins ViewWitness at:

```text
f1930ab2a70175c46d12dd1e61501c3b4ae09408
```

with default features disabled for pure model comparison.

At that exact revision, ViewWitness's optional `egui` feature depends on egui 0.36.2 and publicly exposes direct conversion from `egui::FullOutput` to canonical `Witness`.

## Why no adapter duplication is acceptable

M34 already creates the exact observation identity evidence ViewWitness should consume. Re-implementing AccessKit conversion in ViewWright would create a second interpretation of roles, parents, bounds, visibility, and `NodeIdentity.author_id`.

The intended architecture is:

```text
viewwright-egui FullOutput
    -> ViewWitness's adapter
    -> canonical Witness
```

not:

```text
viewwright-egui FullOutput
    -> ViewWright compatibility witness
    -> maybe ViewWitness later
```

## Canonical behavior to preserve

The migration must preserve all accepted source and renderer semantics, especially:

- M21 fixed-plus-grow geometry;
- M31 axisless overlay geometry/order;
- M32 clip vs scroll_y behavior;
- M34 screen -> region -> semantic-element AccessKit identity hierarchy;
- M36 uniqueness of the observable authored-ID namespace.

## Comparator boundary

M35 remains pure and unchanged. Its normal ViewWitness dependency must stay model-only. M37's egui feature use belongs in test/dev compatibility evidence, not in the comparator's production API.

## Visible pressure

A toolkit upgrade can alter default widget metrics, native rendering details, scrolling behavior, or accessibility output even when source compiles. Therefore compile success alone is insufficient. Existing geometry/accessibility regressions plus representative human preview QA are required before acceptance.

## Canonical non-changes

M37 requires no changes to:

- TOML authoring;
- resolved blueprint semantics;
- expectation schema/YAML;
- comparator result vocabulary;
- ViewWitness model;
- responsive-layout roadmap.
