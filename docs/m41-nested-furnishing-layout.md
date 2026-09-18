# M41 — Nested Furnishing Layout

## Pressure

M40 proved that ViewWright 0.1 can author Lantern Leaf's major shell but cannot author its local interior structure honestly.

Concrete failures:

- the reader toolbar had to become a fake first-class region merely to sit above the document;
- toolbar content was one flat command flow, squeezing chapter status at the right edge;
- the TTS surface was one flat command region rather than media / transport / choice groups;
- the Voice command received almost no horizontal space and stacked vertically;
- inspector settings remained neighboring property sheets rather than authored local groups.

The problem is structural, not a missing widget list.

ViewWright has screen-scale `composition` and semantic `region`, then jumps directly to a flat list of elements.

M41 adds the missing layer between region and element.

## Product distinction

Three levels must remain distinct:

```text
composition
    screen-scale topology among major regions

region
    major semantic/product surface

furnishing
    local authored structure inside one region

element
    semantic UI object
```

A furnishing is not:

- a fake region;
- a visible panel by default;
- a CSS box;
- an application component runtime;
- a new accessibility-semantic object in M41.

It is structural authored intent analogous to a composition, but scoped below one region.

## Required capability

A region may opt into one furnishing root.

A furnishing tree may express:

- `row` or `column`;
- ordered children;
- spacing-token gap;
- spacing-token padding;
- fixed logical width and/or height;
- non-negative grow weight;
- `clip` or `scroll_y` overflow.

Branch furnishings contain furnishing children.

Leaf furnishings contain element children.

Mixing furnishing and element children in the same furnishing is rejected in M41. Authors can introduce an explicit leaf furnishing instead.

This deliberate restriction keeps local allocation deterministic and prevents M41 from becoming a generic box model.

## Local allocation

Branch furnishing children use the same accepted fixed-plus-grow rule as major layout:

```text
available main axis
- gaps
- fixed child bases
= remaining

remaining distributed by positive grow weight
```

A child may have both a fixed base and grow, preserving M21 semantics locally.

Cross-axis behavior is fill.

M41 does not add intrinsic-size negotiation to the backend-independent planner.

For a child furnishing participating in a branch:

- a horizontal parent uses child `width` + `grow`;
- a vertical parent uses child `height` + `grow`;
- an omitted fixed size with zero grow is invalid because it would have no deterministic slot.

The furnishing root receives the owning region rectangle and therefore must not author its own root width/height/grow.

## Region content mode

Existing regions remain valid and unchanged.

A region without a furnishing root retains the legacy flat renderer path.

A region with a furnishing root enters furnished mode:

- every element owned by that region must occur exactly once in the reachable furnishing tree;
- the region does not also render a second legacy flat element list;
- furnishing order becomes the local presentation order;
- fixture targeting remains element-based and unchanged.

This prevents hidden or duplicate rendering.

## Overflow

M41 supports only vertical furnishing scrolling.

The primary north-star use is a fixed reader toolbar above a growing document furnishing whose inner content scrolls vertically.

Horizontal scrolling, pagination, virtualization, sticky headers, and nested scroll coordination are not added.

## Structural identity

Furnishing IDs are authored structural IDs.

They share ordinary declaration uniqueness with the ViewWright source, but they are **not** M34 author IDs in M41.

This mirrors composition IDs:

- useful for deterministic source structure;
- useful for semantic/ASCII/concept projections;
- useful for LayoutPlan lookup;
- not asserted as observed semantic identity.

An exact `screen.id == furnishing.id` remains legal for the same reason `screen.id == composition.id` is legal: furnishing identity is not in the observable screen/region/element author-ID namespace.

## Verification boundary

M41 must preserve M33/M34/M35 exactly.

Semantic elements continue to have:

- their existing `region` owner in the resolved model;
- their existing `region_author_id` in M33;
- their immediate AccessKit author-identity parent as the owning region;
- their existing M35 immediate-parent comparison.

Furnishing scopes must not insert an observable author-ID node between region and element.

No expectation version bump.

No comparator change.

No ViewWitness change.

## Lantern Leaf pressure specimen

M41 should add a new furnished specimen rather than rewriting the M40 baseline.

The new specimen should:

- remove the fake `reader_toolbar` region;
- make reader toolbar + document local furnishings of the primary reader region;
- allocate toolbar search and toolbar actions into deliberate slots;
- make the document furnishing the vertical scroll surface;
- split the TTS region into deliberate local media/status, transport, and voice/speed groupings;
- optionally group existing inspector property sheets if useful, without adding new control semantics.

The M40 baseline remains frozen evidence of the pre-M41 limitation.

## Non-goals

M41 does not add:

- slider, toggle, select, tabs, segmented choice, progress, media-summary, color-picker, or icon element kinds;
- typed values for those controls;
- richer document blocks;
- font families;
- new visual palette roles;
- assets;
- responsive variants;
- arbitrary alignment/justify/cascade syntax;
- element geometry in M33;
- furnishing author IDs in AccessKit;
- React/Tauri output;
- M42 work.
