# M31 — Overlay Composition Pressure

M13 deliberately rejected `stack` and `overlay` because ViewWright had no accepted pressure establishing overlap geometry, z-order, hit-testing, or projection semantics. M31 introduces that pressure explicitly rather than widening the composition enum speculatively.

The pressure specimen is:

- `specimens/overlay-command-palette-pressure.toml`

It represents a Project Browser workspace with an always-open centered command palette floating above the ordinary workspace. The screen is intentionally static: M31 does not model the event that opened the palette or the event that dismisses it.

## Problem

Current composition kinds are linear only:

- `split`
- `row`
- `column`

They allocate siblings into disjoint horizontal or vertical slots. A floating surface above an otherwise intact workspace cannot be expressed truthfully through those semantics. Modeling the palette as another row/column child would incorrectly consume layout space and move the underlying workspace.

## M31 contract

M31 re-admits exactly one previously deferred kind:

- `overlay`

`stack` remains unsupported.

The initial overlay form is intentionally narrow and two-layered:

1. child 0 is the base composition;
2. child 1 is the floating region.

The base composition fills the overlay composition's inner rectangle.

The floating region uses its authored fixed `width` and `height` and is centered within the same inner rectangle. It does not consume space from the base layer.

The floating region is painted after the base composition and is therefore visually above it. Within overlapping geometry, the floating layer owns pointer interaction precedence. Outside the floating rectangle, the base layer remains ordinarily interactive.

## Source constraints

For `kind = "overlay"`:

- exactly two children are required;
- the first child must resolve to a composition;
- the second child must resolve to a region;
- `axis` must be omitted;
- `gap` must be omitted;
- the floating region must author both fixed `width` and fixed `height`;
- the floating region must not rely on positive `grow` sizing under the overlay parent;
- ordinary composition `padding` remains legal and defines the common inner rectangle used by both layers.

The overlay composition's own `grow` remains parent-relative sizing intent if that overlay is itself placed inside a linear parent. M31 does not reinterpret that existing field.

## Resolved semantics

Resolved topology must represent the lack of an axis honestly. Do not assign a fake horizontal or vertical axis to an overlay merely to preserve an old struct shape.

A reasonable resolved representation is equivalent to:

```text
ResolvedComposition
  kind: Overlay
  axis: None
  children:
    - Composition(base)
    - Region(floating)
```

Linear compositions retain their existing typed axis semantics.

## Layout semantics

Given overlay inner rectangle `R`:

- base composition rectangle = `R`;
- floating region width = authored fixed width;
- floating region height = authored fixed height;
- floating region x/y = centered in `R`;
- no sibling gap is applied;
- the two child rectangles may overlap by design.

LayoutPlan remains backend-independent and ID-addressable.

M31 does not add arbitrary x/y coordinates, anchors, edge attachment, percentages, min/max sizing, constraints, or responsive overlay placement.

## Rendering and interaction semantics

The backend must render the base first and floating region second.

The floating region's visual surface and children use ordinary authored region/element semantics. M31 does not introduce a special dialog widget.

When pointer geometry overlaps, the later floating layer must have interaction precedence over underlying widgets in that area. This is overlap hit-testing fidelity, not modality.

Outside the floating rectangle, underlying controls may still interact.

## Projection semantics

Semantic/debug, ASCII, and concept projections must expose `overlay` as real topology rather than flattening it into a row/column or hiding the floating layer.

Because overlay has no axis, projections must not invent one.

## Explicit non-goals

M31 does not add:

- `stack`;
- modal behavior;
- backdrop dimming;
- focus trapping;
- dismissal behavior;
- keyboard routing;
- anchoring to another element;
- popover placement;
- arbitrary absolute coordinates;
- draggable/resizable windows;
- docking;
- multiple floating layers;
- conditional fixture-driven structure;
- application event execution;
- a general window manager.

## Acceptance signal

M31 succeeds when the pressure specimen resolves into a two-layer overlay, the base workspace retains its ordinary geometry, the fixed command-palette surface is centered above it, projections preserve overlay topology without a fake axis, and the egui preview demonstrates correct visible and interaction layering.
