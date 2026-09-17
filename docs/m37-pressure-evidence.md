# M37 — Pressure Evidence

M37 is earned by an attempted composition of already accepted milestones.

## M33

M33 can build a deterministic intended expectation from `ResolvedBlueprint + logical viewport`.

## M34

M34 emits exact authored screen/region/element identities into the real egui AccessKit tree.

Existing renderer tests already produce AccessKit-enabled real `egui::FullOutput` values and inspect those anchors directly.

## ViewWitness adapter

Pinned ViewWitness revision `f1930ab2a70175c46d12dd1e61501c3b4ae09408` publicly exposes:

```text
EguiCaptureContext
witness_from_egui_output
witness_from_egui_tree_update
```

Its `egui` feature is intentionally optional.

## M35

M35 depends on ViewWitness with `default-features = false`, so the comparator consumes only the canonical model and does not pull in ViewWitness's egui toolkit dependency.

That was correct for a pure comparator, but it allowed toolkit incompatibility to remain latent.

## Concrete incompatibility

Current ViewWright workspace:

```text
egui = 0.31
eframe = 0.31
```

Pinned ViewWitness adapter:

```text
egui = 0.36.2
```

Rust therefore treats the two `egui::FullOutput` types as different package-version types. Direct adapter composition cannot compile.

## Why alignment is the narrow fix

Rejected alternatives:

- duplicating ViewWitness's AccessKit conversion inside ViewWright;
- serializing an egui tree merely to cross package versions;
- introducing a ViewWright-specific witness shim;
- weakening M35 to consume some second ad-hoc observation format;
- mutating ViewWitness solely to support ViewWright's older toolkit.

Those approaches create semantic duplication or new bridge formats where a shared toolkit version is sufficient.

## Why M37 stops before comparison

Once real ViewWright output can be converted by ViewWitness, a distinct next pressure can prove M33 expectation -> M34 observation -> ViewWitness -> M35 exact comparison. Keeping that proof separate makes any failures attributable: toolkit compatibility first, semantic comparison second.
