# M4 — Implementation Boundary

M4 is a geometry-fidelity milestone, not a general layout-system expansion.

## In scope

- composition `grow`
- consistent grow validation
- a pure/backend-independent major-slot layout projection
- stable composition/region rectangles keyed by semantic IDs
- fixed main-axis sizes
- proportional growth
- true composition padding inset
- sibling gaps
- recursive nested composition layout
- cross-axis fill
- egui consumption of the resulting major slots
- region surfaces filling their assigned slot
- tests at concrete viewport sizes

## Model/backend boundary

The model owns authored/resolved semantic sizing intent.

The layout projection owns deterministic rectangle derivation for major compositions/regions at a concrete viewport.

The egui backend owns rendering elements inside already-allocated region rectangles.

Do not move egui rectangles/types into `viewwright-model`.

Do not move button/text measurement or widget placement into the backend-independent layout plan.

## Prefer a small pure layout layer

If introduced as a crate/module, it should be callable in tests without launching egui.

Inputs should be plain resolved semantics + logical viewport dimensions.

Outputs should be plain logical geometry.

The projection should be deterministic.

## Do not broaden

M4 must not become:

- a full layout engine for arbitrary widgets
- flexbox/grid recreation
- responsive/adaptive authoring
- drag-resizable split panes
- docking
- scroll containers
- intrinsic cross-backend measurement
- text layout
- animation
- runtime state
- ViewWitness comparison

If a current specimen cannot be handled without one of these, stop and report the pressure instead of silently expanding scope.

## Existing behavior

Preserve accepted M0–M3 parsing, validation, semantic tree, ASCII, concept specification, fixture content, and visual semantics.

The layout change must not reintroduce renderer-specific app IDs or fixture-name heuristics.

## Runtime discipline

Layout math is lightweight. Parsing/resolution/file I/O remain outside the render hot path.

Automated QA must close native windows/processes before completion unless explicitly told otherwise.