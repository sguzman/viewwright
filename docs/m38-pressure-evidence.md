# M38 — Pressure Evidence

M38 is not speculative. Every required stage already exists and is accepted.

## M33

`viewwright_expectation::build_expectation` already builds typed intended expectations from a `ResolvedBlueprint` and explicit logical viewport.

Its region bounds come from backend-independent `LayoutPlan`.

## M34

The egui renderer publishes exact authored screen, region, and semantic-element IDs through AccessKit `author_id`.

Region identity anchors carry planned region bounds, and element anchors are immediate semantic children of their owning region.

## M35

`viewwright_compare::compare` already compares:

- logical viewport width/height;
- exact screen/region/element author-ID presence;
- exact region bounds where observed;
- element ownership through immediate parent author identity.

Exact success already means no mismatches and no evidence gaps.

## M37

M37 aligned ViewWright with egui/eframe 0.36.2 and proved that real AccessKit-enabled ViewWright `egui::FullOutput` passes directly into the pinned ViewWitness egui adapter.

The resulting Witness validates and preserves representative M34 identities.

M37's own authority explicitly names exact expectation-vs-real-Witness verification as the next pressure.

## Remaining gap

No test currently executes:

```text
build_expectation
    +
real viewwright_egui::show
    -> witness_from_egui_output
    -> compare
```

Synthetic M35 Witness construction proves comparator logic but cannot prove that the accepted renderer/adapter composition actually satisfies the accepted expectation.

M37 proves real Witness conversion but intentionally stops before comparison.

M38 closes exactly that remaining gap.

## Why responsive layout is not M38

`docs/roadmap-responsive-layout-authoring.md` explicitly leaves responsive authoring unnumbered until a concrete screen requires two intentionally different layouts that the current single-topology model cannot honestly represent.

That promotion condition has not been established by the accepted M37 work.

The end-to-end verification gap is explicit and already earned.
