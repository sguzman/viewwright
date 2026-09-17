# M37 — Schema Summary

M37 adds no ViewWright authoring schema and no resolved-model schema.

No changes are authorized to:

- TOML shape;
- `ResolvedBlueprint` fields;
- `LayoutPlan`;
- M33 `ViewWrightExpectation` schema/version;
- M35 `ComparisonReport` schema;
- ViewWitness `Witness` schema.

The only contract change is dependency/toolkit compatibility at the egui observation boundary.

## Dependency target

ViewWright workspace:

```text
egui   = 0.36.2
eframe = 0.36.2
```

Pinned ViewWitness authority remains:

```text
revision = f1930ab2a70175c46d12dd1e61501c3b4ae09408
```

A test-only dependency may enable `viewwitness` feature `egui` with default features disabled.

Do not add observer/showcase features or a second observation model.
