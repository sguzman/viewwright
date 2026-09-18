# M42 — Implementation Plan

## 1. Model

Add semantic kinds:

- Choice;
- Boolean;
- Scalar.

Add typed choice configuration with select/segmented presentation and local option IDs/labels.

Add typed scalar configuration with min/max/step/unit.

Require action IDs for Command and all M42 controls; preserve command behavior.

Add typed fixture states.

Require every fixture to cover every M42 control exactly once.

Add negative tests for configuration/state misuse.

## 2. RenderState and interactions

Add fixture-scoped ephemeral state maps for M42 controls.

Initialize lazily from the active fixture's typed seed.

Clear/reset with existing RenderState reset semantics.

Extend interaction output with optional typed value.

Commands emit None.

M42 controls emit the changed typed value.

Do not persist outside renderer state.

## 3. egui

Render:

- choice/select with finite options;
- choice/segmented with selectable segmented presentation;
- boolean with a boolean affordance;
- scalar with bounded step/unit slider.

Keep author identity on the semantic element anchor and preserve immediate region owner identity.

## 4. Projections

Semantic/debug:
- kind/config/action.

ASCII:
- identify control kind/presentation/range without fixture execution.

Concept:
- structural render includes control semantics/config;
- add fixture-aware rendering path that emits selected/current representative control values for one fixture.

Do not broaden this into the full Stage 6 concept dossier.

## 5. Expectation 0.2

Advance canonical build output/version to 0.2.

Add Choice/Boolean/Scalar expectation kinds.

Update deterministic YAML tests/goldens.

Do not change comparison algorithm.

## 6. Lantern Leaf controls specimen

Add:

`specimens/lantern-leaf-reader-controls.toml`

Start from the accepted M41 structure without modifying the M41 file.

Use real M42 control semantics for the required acceptance list.

Keep unsupported color/progress/media/document/visual/responsive ideas explicit rather than faking them.

Register preview.

## 7. Verification

Prove new semantic element author IDs survive AccessKit.

Prove immediate major-region ownership.

Run real exact verification for M42 specimen.

Keep all earlier cases green under expectation 0.2.

## 8. Interaction tests

Programmatically change at least:

- one select choice;
- one segmented choice;
- one boolean;
- one scalar.

Assert typed action/value output and local-state persistence/reset behavior.

## 9. Human QA

Launch M41 furnished and M42 controls specimen.

Human verifies semantic/visual improvement and manipulates one control of each M42 family.

Preview host should display the last action plus typed changed value clearly enough to inspect.

## 10. Checks

Run:

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

## 11. Publish

README -> implementation complete / human-Director review pending.

Post evidence to M42 issue and leave open.

Commit/push/sync/clean.

Do not start M43.
