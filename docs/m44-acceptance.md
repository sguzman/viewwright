# M44 — Acceptance

## Backward compatibility

- every existing non-responsive source resolves/renders exactly as before;
- existing `layout(blueprint, width, height)` call sites remain valid;
- M40–M43 north-star specimens remain frozen.

## Responsive model

- optional responsive source resolves to typed variants;
- width intervals are exhaustive, non-overlapping, deterministic, and order-independent;
- exact boundary semantics are tested;
- default variant root equals screen.root;
- invalid roots/overrides are rejected;
- no runtime parsing.

## Variant validation

Every authored variant is validated, not only whichever one appears during a test render.

For each variant:

- root composition is structurally valid;
- reachable region set is deterministic;
- region overrides target reachable regions;
- dominant target is reachable;
- active furnishing roots resolve;
- active furnished regions cover their semantic elements exactly once;
- same semantic element may occupy different furnishings across variants;
- alternate furnishing trees may be reused across variants for the same region;
- a furnishing cannot semantically belong to two different regions.

## Fixture reachability

Responsive fixture content must be reachable in at least one variant.

Fixture content may be dormant in variants where its element is absent.

Non-responsive M30 behavior remains unchanged.

## Layout

- variant selection occurs from root viewport width before geometry planning;
- region geometry overrides participate in the existing fixed-plus-grow algorithm;
- active root controls topology;
- alternate furnishing root controls local structure;
- inactive regions/compositions/furnishings receive no active plan rectangle;
- `LayoutPlan` exposes selected variant;
- no egui breakpoint logic.

## Projection

- viewport-free structural projections use the explicit responsive default;
- viewport-aware projections report selected variant and active hierarchy;
- concept output makes wide/compact/narrow intent inspectable;
- fixture-aware responsive concept output retains representative M42/M43 state for active objects.

## Expectation / observation

- expectation remains 0.2;
- expectation region/element set follows active responsive reachability;
- stable semantic IDs are unchanged across variants where present;
- M35 algorithm unchanged;
- ViewWitness unchanged;
- real exact-verification succeeds at representative wide, compact, and narrow viewports.

## Lantern Leaf specimen

Add a new responsive specimen rather than mutating M43.

Required authored intervals:

- narrow: [0, 960);
- compact: [960, 1260);
- wide: [1260, infinity).

Required intent:

### wide
- library + reader + inspector;
- persistent TTS;
- visually equivalent major topology to M43.

### compact
- library + reader;
- inspector absent;
- persistent TTS;
- reduced library width.

### narrow
- reader + persistent TTS;
- library/inspector absent;
- alternate reader furnishing avoids simple toolbar crushing;
- alternate TTS furnishing avoids wide three-column crushing.

The reader remains design.dominant in all states.

## Headless responsive QA

Routine M44 acceptance MUST run without a visible native window and without touching the operator's interactive desktop.

Use a real headless `egui::Context` with synthetic `RawInput` frames to exercise the same resolved responsive specimen through a resize sequence spanning both breakpoints.

Required sequence includes at minimum:

- wide at 1440×900;
- compact at 1100×900;
- narrow at 800×900;
- compact again;
- wide again;
- exact-boundary frames at 1260 and 960;
- immediately-adjacent fractional frames on both sides of each breakpoint.

The headless QA bundle must prove:

- selected variant changes at the authored boundaries;
- active root and active semantic region/element set agree with the selected variant;
- surviving M34 author IDs remain stable across frames;
- intentionally absent IDs disappear and reappear without duplicates or stale nodes;
- renderer-local M42 control state survives hide/reappear when ordinary reset semantics do not apply;
- narrow reader and TTS use their authored alternate furnishings rather than wide structures crushed into the viewport;
- all active region/furnishing rectangles stay within their intended viewport/parent geometry;
- paint clipping does not leak product content into unrelated host/outside geometry;
- M42 popup/scroll renderer regressions remain green;
- M43 rich document/spoken-range renderer remains present and readable by existing structural/paint assertions;
- expectation 0.2 → real FullOutput → ViewWitness → M35 remains exact at representative wide/compact/narrow widths.

A deterministic text/JSON QA report should be emitted for Director audit. Offscreen raster captures may also be generated if the chosen test backend supports them without creating a visible window, but pixels are not required for M44 acceptance when the semantic/layout/paint evidence is complete.

Human live-resize QA is NOT required for M44 acceptance.

Interactive preview may be used only if the human explicitly opts in after implementation for subjective product judgment.

## Regression

No visual-role expansion, height rules, container queries, application navigation runtime, or M45 work.
