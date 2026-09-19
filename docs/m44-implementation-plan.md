# M44 — Implementation Plan

## 1. Responsive source and resolution

Add optional responsive source.

Parse typed width intervals and sparse region overrides.

Resolve all variant roots/references up front.

Validate interval coverage independently of source order.

No parsing/cloning of source TOML during rendering.

## 2. Reachability evolution

Generalize structural reachability to accept an arbitrary root composition.

For non-responsive blueprints, preserve current M29/M30 behavior.

For responsive blueprints:

- compute reachable regions for every variant;
- dominant must be present in every set;
- fixture target must be present in at least one set;
- active viewport uses only its set.

Add explicit tests for dormant fixture content and reappearance.

## 3. Furnishing evolution

Generalize furnishing validation from one global region root to variant-local effective roots.

For every active region in every variant:

- choose override furnishing if supplied, otherwise base region furnishing;
- validate same-region element ownership;
- require each region element exactly once in that active tree.

Allow alternate trees to place the same semantic element differently across variants.

Reject a furnishing being assigned to different semantic regions.

Preserve M41 behavior when responsive is absent.

## 4. Selected layout state

Implement a cheap selected-state helper from resolved blueprint + viewport width.

It should expose:

- variant ID;
- active root;
- effective region sizing/furnishing.

Use it in LayoutPlan and projections rather than duplicating selection logic.

## 5. LayoutPlan

Keep public `layout(blueprint, width, height)`.

Internally select variant first.

Plan only the active root/tree.

Use effective region geometry in fixed-plus-grow calculations.

Plan only active furnishings.

Expose active variant ID.

## 6. Renderer

Traverse the active root/regions from LayoutPlan-selected state.

Do not add breakpoint checks in egui.

Absent region/element state is dormant, not destroyed.

Resize back and forth should preserve renderer-local semantic control state where existing rules permit.

## 7. Expectation

Build 0.2 expectation from active viewport state.

Only active regions/elements appear.

Region bounds come from responsive LayoutPlan.

Do not add variant field or bump version in M44.

Keep M35 algorithm unchanged.

## 8. Projections

Preserve historical no-viewport APIs by using responsive.default.

Add viewport-aware forms for ASCII/concept.

Fixture-aware concept at viewport should include active M42 control values and M43 document semantics for reachable objects.

## 9. Lantern Leaf specimen

Add:

`specimens/lantern-leaf-reader-responsive.toml`

Start from M43 semantically, without editing M43.

Declare alternate top-level compositions/furnishings for compact/narrow.

Author:

- narrow [0,960);
- compact [960,1260);
- wide [1260,infinity).

Wide should preserve M43 major topology.

Compact:
- library + reader;
- no inspector;
- persistent TTS;
- library override around 210px.

Narrow:
- reader + persistent TTS;
- no library/inspector;
- alternate reader furnishing with vertical/local structure rather than crushed wide toolbar;
- alternate TTS furnishing suitable for narrow width;
- larger TTS height override if needed.

All existing semantic element IDs remain.

## 10. Preview

Register responsive specimen.

Show the active responsive variant in preview-host chrome for human QA.

Do not inject that label into product AccessKit author identity.

Live resizing should switch variants automatically.

## 11. Tests

Model:
- valid interval coverage;
- gaps rejected;
- overlaps rejected;
- duplicate IDs;
- invalid/default missing;
- invalid root;
- exact boundary selection;
- source-order independence;
- invalid/dead region override;
- invalid furnishing override;
- dominant missing from one variant rejected;
- fixture target absent everywhere rejected;
- fixture target dormant in one variant accepted.

Furnishing:
- alternate roots for same region;
- same element arranged differently by variant;
- per-variant exact coverage;
- cross-region furnishing reuse rejected.

Layout:
- wide/compact/narrow active roots;
- geometry overrides;
- inactive regions absent;
- active variant exposed.

Projection:
- default and viewport-aware variants deterministic.

Identity:
- surviving region/element author IDs stable across resize;
- hidden IDs absent only while variant omits them;
- variant/composition/furnishing IDs do not leak as M34 author IDs.

## 12. Exact verification

Add real exact-verification at representative widths, for example:

- 1440x900 wide;
- 1100x900 compact;
- 800x900 narrow.

Require all:
- valid Witness;
- zero mismatches;
- zero evidence gaps;
- exact match true.

Keep all prior exact cases green.

## 13. Human resize QA

Run one responsive preview and resize live.

Human verifies both sides and exact boundaries of 960 and 1260.

Check no stale nodes/duplicate surfaces and preserve M42/M43 fixes.

## 14. Checks

Run:

```text
cargo fmt --all
cargo fmt --all --check
cargo test --workspace
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
cargo test -p viewwright-egui --test viewwitness_compat
cargo test -p viewwright-egui --test exact_verification -- --nocapture
git diff --check
```

## 15. Publish

README -> M44 implementation complete; human/Director review pending.

Comment Issue #48 with evidence.

Leave issue open.

Commit/push/sync/clean.

Do not begin M45.
