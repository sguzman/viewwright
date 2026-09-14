# M8 — Implementation Boundary

M8 makes an already-authored semantic distinction survive projection. It is not a general layout or component-system milestone.

## In scope

- typed resolved collection presentation;
- validation of supported presentation values;
- rejection of presentation on non-collection elements;
- conservative default of omitted collection presentation to `list`;
- semantic/debug visibility of collection presentation;
- concept visibility of collection presentation;
- egui distinction between list and adaptive-card collections;
- selected-state preservation in both presentations;
- Project Browser and Dependency Workbench regression coverage;
- preservation of M0–M7 behavior.

## Renderer boundary

The egui backend may choose practical implementation details for `adaptive_cards`, including:

- a backend-local preferred/minimum card width;
- ordinary row wrapping based on currently available element-region width;
- backend-local card padding;
- ordinary egui theme/group framing when no ViewWright visual profile exists;
- existing resolved palette/surface/selection semantics when a visual profile exists.

Those details are renderer policy, not new source syntax.

The semantic requirement is only that `adaptive_cards` remain recognizably card-based and wrap peers across available width rather than silently using the list renderer.

## No new visual language

Do not add authored fields for:

- card width/height;
- card radius;
- card shadow;
- card border;
- grid gap;
- column count;
- image placement;
- card-specific typography.

If later screens prove those controls are semantically necessary, they can be earned separately.

## No generalized responsive system

Using currently available collection width to choose how many card units fit is local renderer adaptation.

M8 must not introduce:

- viewport breakpoints;
- media queries;
- percentage sizing;
- generalized responsive variants;
- constraint solving;
- layout rules that alter major M4 region/composition geometry.

M4 remains authoritative for major screen slots.

## Fixture boundary

Do not widen collection fixture content.

M8 still knows only:

- item id;
- item label;
- selected item.

Do not introduce arbitrary card-child trees or app-specific project metadata.

## Interaction boundary

Cards remain presentation of existing static collection fixtures.

Do not add:

- item click events;
- selection mutation;
- double-click/open semantics;
- drag/reorder;
- per-card command actions.

M7 remains limited to explicit command elements.

## Typed model boundary

After resolution, renderers should not branch on arbitrary presentation strings.

Prefer an enum or equivalent typed representation.

Source convenience/defaulting belongs in validation/resolution; backend code should consume resolved semantics.

## Stop rather than widen

If implementation appears to require a reusable component DSL, generalized grid system, responsive breakpoint language, rich card-content schema, or collection interaction model, stop and report that pressure instead of silently widening M8.
