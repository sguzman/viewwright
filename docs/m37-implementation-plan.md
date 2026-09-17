# M37 — Implementation Plan

## 1. Establish the dependency baseline

Confirm current authority:

- ViewWright workspace `egui = 0.31`, `eframe = 0.31`;
- pinned ViewWitness revision `f1930ab2a70175c46d12dd1e61501c3b4ae09408`;
- ViewWitness `egui = 0.36.2` behind optional feature `egui`;
- M35 normal dependency keeps `default-features = false`.

## 2. Align toolkit versions

Move ViewWright workspace `egui` and `eframe` to exact 0.36.2 compatibility targets and refresh the lockfile.

Do not opportunistically upgrade unrelated dependencies.

## 3. Perform mechanical API migration only

Adapt `viewwright-egui` and `viewwright-preview` only where egui/eframe API changes require it.

Preserve:

- layout geometry;
- density behavior;
- visual tokens and surfaces;
- overlay semantics;
- overflow semantics;
- search state;
- command activation;
- M34 AccessKit hierarchy, exact author IDs, and region bounds.

Do not redesign code merely because newer toolkit APIs exist.

## 4. Add the narrow compatibility proof

At the narrowest appropriate test boundary, add the pinned ViewWitness dependency with:

```text
default-features = false
features = ["egui"]
```

Render a canonical ViewWright blueprint using a real `egui::Context` with AccessKit enabled and an explicit screen rectangle.

Pass the resulting `egui::FullOutput` directly into:

```text
viewwitness::witness_from_egui_output
```

No intermediate ViewWright observation struct is allowed.

## 5. Validate converted evidence

Assert that:

- conversion returns a Witness;
- `validation_issues()` is empty;
- screen author ID is present exactly;
- representative region and element author IDs are present exactly;
- fixture-local collection/tree IDs remain outside canonical author identity.

Existing M34 direct AccessKit tests remain the detailed hierarchy/bounds authority.

## 6. Dependency audit

Verify the compatibility test graph resolves one egui version at the bridge boundary and that the normal M35 dependency still keeps ViewWitness default features disabled.

Useful evidence may include `cargo tree -d` / feature-tree output.

## 7. Regression and visual QA

Run the full workspace tests and checks.

Because this is a toolkit migration, perform representative visual QA for at least:

- Project Browser;
- visual Reader;
- M31 overlay pressure;
- M32 overflow pressure.

No visible redesign is intended. If meaningful visual drift appears, diagnose it rather than accepting it as an incidental upgrade effect.

Any preview process launched for QA must be closed before completion reporting.

## 8. Bookkeeping

Update README:

- M36 — accepted;
- M37 — implementation complete; Director audit pending.

Leave the M37 issue open for Director audit.

Do not begin the end-to-end comparator milestone.
