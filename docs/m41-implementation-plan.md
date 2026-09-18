# M41 — Implementation Plan

## 1. Sync

Start from the committed M41 authority head.

Read all M41 and v0.2 north-star documents.

Do not modify the v0.1.0 tag or M40 baseline.

## 2. Model

Add:

- optional `region.furnishing`;
- source/resolved furnishing declarations;
- row/column kind;
- typed element/furnishing child references;
- gap/padding;
- width/height/grow;
- clip/scroll_y.

Validate:

- IDs/references;
- spacing tokens;
- finite nonnegative grow;
- fixed pixel sizes;
- homogeneous direct child class;
- unique parents;
- cycles;
- root ownership;
- exact element coverage for furnished regions;
- same-region element ownership;
- all furnishings rooted exactly once.

Preserve all legacy sources.

## 3. Layout

Extend `LayoutPlan` with furnishing rectangles.

After major region geometry is known, plan each furnishing root inside its region rectangle.

Reuse the accepted fixed-plus-grow rule.

Do not add element rectangles.

Add focused tests for:

- row fixed + grow;
- column fixed + grow;
- fixed + grow on same child;
- padding/gap;
- cross-axis fill;
- nested branch/leaf structure.

## 4. Projections

Expose furnishing hierarchy in:

- semantic/debug;
- ASCII;
- concept specification.

Keep the output deterministic.

Do not invent control semantics absent from elements.

## 5. egui

Preserve legacy region rendering exactly when `region.furnishing` is absent.

For furnished regions:

- render only the furnishing root;
- branch nodes create child UIs at planned furnishing rectangles;
- leaf row/column nodes flow existing elements;
- authored gap replaces the leaf's legacy implicit inter-element gap;
- furnishing padding defines local inset;
- furnishing scroll_y uses vertical ScrollArea;
- do not paint furnishing frames by default.

Maintain element accessibility parent as the owning region anchor even though egui scopes are nested.

## 6. Lantern Leaf furnished specimen

Add a new specimen, expected name:

`specimens/lantern-leaf-reader-furnished.toml`

Do not modify the M40 baseline.

Use current semantic elements/fixtures only.

Required structural improvements:

- reader region owns a furnishing column;
- toolbar is a local furnishing, not a region;
- toolbar contains separate search and actions slots;
- document owns the growing/scrolling slot;
- TTS region owns local left/center/right furnishing groups;
- right voice/speed group has deliberate space;
- use inspector furnishing only if it demonstrates structure without adding fake semantics.

## 7. Identity/verification tests

Prove:

- furnishing IDs absent from AccessKit author IDs;
- semantic elements still immediate authored-identity children of region;
- new furnished specimen produces valid real Witness;
- M35 exact comparison has zero mismatches/evidence gaps;
- all M37/M38/M40 exact cases remain green.

## 8. Human QA

Launch both M40 baseline and M41 furnished specimen at comparable desktop size.

Human checks:

- fake toolbar region eliminated;
- toolbar status no longer clipped/squeezed;
- TTS Voice no longer vertical;
- shell/dominance preserved;
- no unexpected legacy regressions.

Close preview after QA unless human is actively inspecting it.

## 9. Checks

Run at minimum:

```text
cargo fmt --all
cargo fmt --all --check
cargo test --workspace
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p viewwright-egui --test viewwitness_compat
cargo test -p viewwright-egui --test exact_verification -- --nocapture
git diff --check
```

Run focused M41 model/layout/projection tests too.

## 10. Publish

Update README to implementation-complete / Director-human QA pending.

Comment the M41 issue with implementation evidence.

Leave the issue open.

Commit/push/synchronize/clean.

Do not start M42.
