# M9 — Implementation Boundary

M9 makes existing screen-density intent survive projection. It is not a second layout engine and not a new visual-authoring system.

## In scope

- typed resolved screen density;
- validation of supported density values;
- Comfortable default when omitted;
- semantic/debug visibility;
- concept visibility;
- a small egui-local density policy;
- A/B pressure specimens differing only in density;
- canonical regression coverage;
- preservation of M0–M8 behavior.

## Backend micro-layout boundary

The egui backend may map density to a small deterministic set of local metrics such as:

- region content inset;
- vertical item spacing;
- button/control padding;
- minimum interaction height;
- element-local gaps;
- card-local vertical compactness where appropriate.

These values are backend policy.

The policy should be centralized enough to test directly and avoid scattering `if dense` branches throughout renderer code.

## Major layout remains M4

Density must not alter:

- LayoutPlan rectangle calculation;
- composition gap/padding values resolved from authored tokens;
- region width/height/grow;
- composition growth;
- viewport allocation.

For identical blueprints that differ only in density, M4 layout plans must remain identical for the same viewport.

## Typography remains M2

Density must not change authored type-scale values.

Do not shrink text merely to make Dense fit more content.

## Visuals remain M2/M6

Density must not change:

- palette;
- border policy;
- corner radius;
- surface roles;
- visual-audit thresholds.

## Content remains M3/M5/M7/M8

Density must not alter:

- fixture payloads;
- selected state;
- tree hierarchy;
- command enabled state;
- action ids;
- collection presentation.

## No authored density metrics

Do not add TOML fields such as:

- `item_gap_dense`;
- `control_height`;
- `region_inset`;
- `density_scale`;
- per-region density.

If future screens require authored control over those dimensions, earn that vocabulary separately.

## No global egui mutation

Apply density inside the authored screen scope. Do not permanently mutate host-preview chrome or global Context styling.

## Render-thread rule

Density application is tiny style/layout arithmetic and is safe on the render thread.

Do not introduce parsing, I/O, allocation-heavy preprocessing, or business work into the frame loop.

## Stop rather than widen

If implementation appears to require a generalized responsive system, token rewrite, layout scaling engine, per-region overrides, or a typography redesign, stop and report the pressure instead of silently widening M9.
