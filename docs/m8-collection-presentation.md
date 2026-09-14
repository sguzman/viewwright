# M8 — Collection Presentation Fidelity

ViewWright already authors collection presentation intent, but the accepted egui backend currently discards it.

The canonical Project Browser contains two collection elements with intentionally different presentations:

```toml
[[element]]
id = "navigation_items"
kind = "collection"
presentation = "list"

[[element]]
id = "project_collection"
kind = "collection"
presentation = "adaptive_cards"
```

Today both are rendered as the same vertical sequence of labels. The source says one thing while the backend silently does another.

M8 makes this existing authoring field semantically real.

## Goal

Preserve collection presentation intent through:

```text
source `presentation`
    ↓
validated typed collection presentation
    ↓
resolved semantic element
    ├── semantic/debug projection
    ├── concept projection
    └── egui presentation
```

M8 introduces no new source field.

## Initial presentation vocabulary

M8 supports exactly:

- `list`
- `adaptive_cards`

These are semantic presentation modes, not serialized egui widgets.

### `list`

A collection whose items form a linear reading/navigation sequence.

The egui backend should preserve the existing compact vertical treatment.

### `adaptive_cards`

A collection whose peer items should read as discrete card-like units and wrap across the available collection width.

The semantic promise is:

- peer items are visually discrete;
- items flow into multiple columns when useful space permits;
- items wrap to additional rows as needed;
- the backend may choose practical card dimensions;
- the presentation remains recognizably card-based at narrower widths rather than silently becoming the ordinary list renderer.

This is intentionally much smaller than a responsive-layout language.

## Typed semantics

Resolved collection presentation should be typed rather than retained as an unchecked arbitrary string.

A shape such as:

```text
CollectionPresentation
  List
  AdaptiveCards
```

is appropriate.

Exact Rust naming is an implementation choice.

## Compatibility rules

For M8:

- `presentation` is valid only for `kind = "collection"`;
- unknown presentation values are validation errors;
- collection elements without `presentation` may resolve to `list` as the conservative default;
- accepted canonical collections should keep their explicit authored values;
- non-collection elements must not silently carry unused presentation strings.

Do not infer presentation from element ids, labels, regions, fixture ids, or screen ids.

## Fixture boundary

Collection fixture content remains exactly the M3 content model:

- stable item id;
- item label;
- optional selected item.

M8 does not add card metadata, subtitles, icons, images, badges, actions, or arbitrary card children.

The same fixture content may be projected differently according to the collection's presentation semantics.

## Selection

Existing fixture-backed `selected` state remains representative static preview state.

For `adaptive_cards`, the selected item should remain visibly distinguishable using existing backend/visual semantics.

M8 does not add collection-selection interaction events.

## Visual boundary

Project Browser currently has no resolved visual profile, so `adaptive_cards` must work reasonably with ordinary egui theme defaults.

When a visual profile is present, the backend may use existing resolved palette/surface/selection semantics.

Do not introduce new card-specific color, shadow, border, radius, or spacing syntax in M8.

## Canonical pressure

The primary pressure specimen is the already-accepted:

- `project_browser / many_projects`

Expected distinction:

- `navigation_items` remains a compact vertical list;
- `project_collection` becomes a wrapping card collection;
- the selected project remains visually identifiable.

Additional regression pressure:

- `project_browser / selected_project` — one selected card;
- `project_browser / empty` — no fabricated cards;
- `dependency_workbench / healthy` — Scope and Dependencies remain lists;
- `dependency_workbench / advisory` — list semantics remain intact.

## Governing principle

> If ViewWright authors a presentation distinction, a backend must either honor it or diagnose inability to honor it. It must not silently erase the distinction.
