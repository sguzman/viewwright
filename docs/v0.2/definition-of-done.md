# ViewWright v0.2 definition of done

v0.2 is not complete merely because more widget kinds exist.

The release is complete when ViewWright can honestly author the durable design intent of the Lantern Leaf north star.

## Canonical north-star source

There is one canonical ViewWright north-star design (or one explicit source family if responsive representation requires it) that preserves stable semantic identity across its intended states.

It is not a copy of Lantern Leaf's experimental furnishing syntax.

## Furnishing fidelity

The design can represent without fake major-region inflation:

- reader toolbar and control clusters;
- grouped inspector settings;
- persistent TTS transport structure;
- local layout among semantically grouped objects.

## Control fidelity

Representative preview states can express the meaning and current value/state of the controls the north star actually uses.

The model remains above toolkit-specific widgets.

## Reading fidelity

The reading surface can distinguish meaningful document blocks and representative spoken/highlight state.

Reading presentation can express the durable typography/layout intent needed by the concept.

## Visual fidelity

The design can preserve the concept's important palette and UI-vs-reading hierarchy without arbitrary per-widget CSS-style bags.

## Responsive fidelity

The north-star screen has authored, deterministic `wide`, `compact`, and `narrow` behavior.

At exact breakpoints:

- rule selection is deterministic;
- all variants validate;
- LayoutPlan is backend-independent;
- semantic identity survives where applicable;
- human live-resize QA confirms the authored behavior rather than accidental compression.

## Concept-art fidelity

A deterministic concept dossier generated from the canonical design contains enough design information for a human or external image generator to create a recognizable Lantern Leaf-like concept without inventing the major structure.

Generated imagery is non-authoritative.

Useful visual changes discovered through image review are authored back into ViewWright source.

## Verification fidelity

Representative responsive states/viewports can still participate in the existing expectation → real renderer → ViewWitness → exact comparison chain.

## Explicit non-requirements

v0.2 does not require:

- a React/Tauri renderer backend;
- TTS execution;
- EPUB parsing;
- application persistence;
- arbitrary data binding;
- CSS compatibility;
- automatic image generation inside the core library.

Those may be separate future pressure.
