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

## Live resize human QA

Human resizes the same preview window across both exact breakpoints.

Verify:

- >=1260 displays wide;
- below 1260 switches compact deliberately;
- below 960 switches narrow deliberately;
- exactly 1260 is wide;
- exactly 960 is compact;
- resizing back restores stable semantic UI;
- no stale/duplicated regions;
- no accidental fixed-column squeeze masquerading as responsive behavior;
- narrow reader/TTS remain usable;
- existing M42 popup/scroll fixes survive;
- M43 rich document remains readable.

## Regression

No visual-role expansion, height rules, container queries, application navigation runtime, or M45 work.
