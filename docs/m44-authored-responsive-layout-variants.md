# M44 — Authored Responsive Layout Variants

## Goal

Make major window-resize behavior authored ViewWright intent instead of accidental backend compression.

M44 introduces a small deterministic responsive language for root viewport width.

The pipeline becomes:

```text
resolved semantic blueprint
        +
root logical viewport width
        ↓
responsive variant selection
        ↓
active authored topology / region overrides
        ↓
LayoutPlan
        ↓
backend projection
```

Selection happens before backend rendering.

## Why now

M40–M43 established:

- a truthful major shell;
- local furnishing;
- semantic settings controls;
- rich reader document semantics.

The accepted wide Lantern Leaf screen is stable enough that alternate layouts can now preserve the same semantic objects rather than encoding temporary structural mistakes multiple times.

At narrow widths the current model has no language for deliberate change. Fixed library and inspector widths, fixed toolbar slots, and the TTS furnishing are merely compressed.

That satisfies the responsive-roadmap promotion condition.

## Width-only first milestone

M44 evaluates only the **root logical viewport width**.

No height conditions.
No nested/container queries.
No device or platform queries.
No aspect-ratio conditions.

Those require later concrete pressure.

## Variant selection

A responsive blueprint authors named variants.

Each variant has:

- ID;
- composition root;
- optional inclusive `min_width`;
- optional exclusive `max_width`;
- zero or more sparse region overrides.

Omitted `min_width` means 0.

Omitted `max_width` means unbounded.

Authored breakpoint values are non-negative integer logical pixels.

Variant intervals must:

- be non-empty;
- never overlap;
- exactly cover every width from 0 through unbounded infinity;
- meet exactly at boundaries.

Source order is not precedence.

There is no fallback cascade.

For a width W, exactly one interval contains W.

## Exact boundary semantics

```text
min_width <= W < max_width
```

when both exist.

Example:

```text
narrow   [0, 960)
compact  [960, 1260)
wide     [1260, infinity)
```

Therefore:

- 959.999 logical px selects narrow;
- exactly 960 selects compact;
- exactly 1260 selects wide.

## Default variant

Responsive source names one `default` variant.

The default exists for APIs that intentionally have no viewport argument, such as the historical structural concept/ASCII projections.

The default variant's root MUST equal `screen.root`.

It is not a fallback for uncovered widths; uncovered widths are invalid because interval coverage must be exhaustive.

## Alternate topology

Variants select top-level composition roots already declared in the blueprint.

This deliberately reuses the existing composition language rather than adding responsive-only container syntax.

Alternate roots may:

- omit regions;
- reorder regions;
- place the same regions in a different composition topology.

A semantic region/element keeps the same authored ID whenever it survives into another variant.

Do not duplicate semantic regions/elements merely to change layout.

## Region overrides

M44 permits sparse per-variant overrides of:

- `width`;
- `height`;
- `grow`;
- `furnishing`.

These are layout/presentation overrides only.

M44 does not responsively override:

- role;
- importance;
- surface;
- overflow;
- semantic element kind;
- labels/actions;
- fixture values;
- palette;
- screen density.

An override on a region absent from that variant is invalid dead intent.

M44 does not provide "unset" syntax. An override supplies a concrete replacement value.

## Responsive furnishing

A variant may select a different furnishing root for an existing semantic region.

This is needed when the same reader/TTS surface requires different local organization at narrow width.

Alternate furnishings are ordinary top-level furnishing declarations.

They do not become responsive-only semantic objects.

Within each variant:

- a furnished region's active furnishing tree must cover every element owned by that active region exactly once;
- an element may therefore have different furnishing parents in different variants;
- a furnishing may be reused across variants for the same semantic region;
- one furnishing tree may not serve two different semantic regions.

M41's old single-root/global-parent assumptions evolve to variant-local validation.

## Reachability

Each responsive variant has its own structurally reachable region set.

### Dominant target

`design.dominant` must remain reachable in **every** variant.

A screen cannot author a dominant object that disappears at a breakpoint.

### Fixture content

For responsive blueprints, fixture content must target an element reachable in **at least one** variant.

Fixture content may be dormant in a variant where that element's region is intentionally absent.

Dormant fixture content:

- remains valid blueprint data;
- is not rendered;
- is not exported into the active viewport expectation;
- becomes active again if a later variant contains the semantic element.

This is the deliberate responsive evolution of M30.

Non-responsive blueprints preserve the exact M30 rule based on `screen.root`.

### M42 control coverage

M42's fixture-state coverage remains blueprint-wide.

A responsive blueprint still explicitly seeds every semantic value control, even if that control is dormant in one variant.

This keeps representative state stable when the control disappears and later reappears.

## Runtime state

Renderer-local state remains keyed by semantic element identity and fixture, not by responsive variant.

If a control is changed, hidden by resize, and later reappears in the same fixture, its renderer-local value remains unless existing reset behavior clears it.

No responsive variant owns application state.

## Identity

M44 adds no new M34 semantic author IDs.

Variant IDs, alternate composition IDs, and furnishing IDs remain structural identity.

Screen/region/element author IDs remain unchanged.

A region/element absent in the active variant simply has no observed node for that viewport.

## Backend independence

Variant selection and active structural resolution occur before egui.

`viewwright-layout` owns deterministic selection/planning.

The egui renderer consumes the active plan; it must not implement private breakpoint logic.

## No responsive inference

The renderer must never decide:

- "this is narrow, hide inspector";
- "stack this toolbar because it looks cramped";
- "turn this into mobile mode".

Those decisions come only from authored variants.

## North-star responsive behavior

The M44 Lantern Leaf pressure specimen authors:

### wide — [1260, infinity)

- existing M43 wide composition;
- library + reader + inspector;
- persistent TTS surface;
- existing M43 reader/TTS furnishing.

### compact — [960, 1260)

- library + reader;
- inspector intentionally absent;
- persistent TTS surface;
- library width reduced through a region override;
- wide reader/TTS furnishing may remain if it fits honestly.

### narrow — [0, 960)

- reader remains dominant;
- library and inspector intentionally absent;
- persistent TTS remains;
- alternate narrow reader furnishing;
- alternate narrow TTS furnishing;
- TTS region receives larger authored height as needed.

This is a pressure design, not a universal responsive recipe.

Future product pressure may replace hidden surfaces with drawers/overlays; M44 does not invent those interaction patterns.

## Non-goals

M44 does not add:

- height breakpoints;
- container queries;
- CSS cascade or specificity;
- automatic media-query generation;
- device sniffing;
- animation/transitions;
- arbitrary constraint solving;
- responsive palette/type changes;
- responsive semantic kind changes;
- responsive fixture values;
- drawers/tabs/navigation runtime;
- visual-role expansion;
- M45.
