# M37 — egui / ViewWitness Compatibility Pressure

## Problem

M33, M34, M35, and M36 establish the semantic verification chain on paper and in isolated tests:

```text
ViewWright expectation
    -> exact authored observation identity
    -> ViewWitness Witness
    -> exact comparison
```

The next pressure is to prove that a real ViewWright egui frame can cross into the already-pinned ViewWitness egui adapter.

That composition is currently blocked by toolkit version identity:

- ViewWright uses `egui` / `eframe` 0.31;
- pinned ViewWitness revision `f1930ab2a70175c46d12dd1e61501c3b4ae09408` declares its egui capture adapter against `egui` 0.36.2;
- `viewwitness::witness_from_egui_output` therefore accepts a different Rust `egui::FullOutput` type from the one ViewWright currently produces.

M35 did not expose this because it deliberately depends on ViewWitness model-only with default features disabled.

## Principle

Cross-project verification must use one real toolkit type boundary. ViewWright must not copy, reinterpret, serialize around, or hand-translate ViewWitness's egui adapter merely to hide a dependency mismatch.

## Goal

Align ViewWright's egui/eframe dependency line with the pinned ViewWitness revision and prove direct conversion of an actual AccessKit-enabled ViewWright `egui::FullOutput` through ViewWitness's public egui capture adapter.

The compatibility proof must use the pinned ViewWitness revision already audited by M35.

## Exact compatibility target

For M37, ViewWright should pin its workspace `egui` and `eframe` dependencies to 0.36.2 so the integration graph can resolve the same egui package version used by the pinned ViewWitness adapter.

A dev/test-only ViewWitness dependency may enable only the `egui` feature necessary to call:

```text
viewwitness::witness_from_egui_output
```

Do not enable ViewWitness observer, showcase, network, or eframe features merely for this proof.

## What the proof establishes

At least one canonical ViewWright blueprint must be rendered through the real `viewwright-egui` renderer in an AccessKit-enabled egui context at an explicit logical viewport. The resulting real `egui::FullOutput` must pass directly into the pinned ViewWitness adapter without an intermediate compatibility struct.

The resulting `Witness` must validate and preserve representative M34 authored identities across screen, region, and semantic-element anchors.

This milestone does **not** yet run the M35 comparator over that witness. Exact expectation/witness end-to-end verification is the next pressure after toolkit compatibility is established.

## Visible-behavior requirement

The dependency alignment has no intended product-design delta. Existing ViewWright layout, styling, interaction, overflow, overlay, density, and accessibility semantics should remain materially unchanged.

Toolkit API migrations may require mechanical source changes. They do not authorize design changes.

## Non-goals

M37 does not add:

- new TOML fields;
- new resolved-model fields;
- new layout semantics;
- responsive layout;
- a ViewWright verification public API;
- M35 comparator integration with real egui output;
- ViewWitness network capture;
- `egui_inspection`;
- capture servers;
- screenshots as machine evidence;
- paint evidence;
- fixture expectation semantics;
- element geometry expectations;
- ViewWitness repository changes;
- M38 work.
