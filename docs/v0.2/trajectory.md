# ViewWright v0.2 trajectory

## North star

v0.2 works toward faithful authorship of the Lantern Leaf EPUB + TTS reader concept.

The work is intentionally sequenced so each abstraction is earned by concrete pressure.

## Stage 0 — honest v0.1 baseline

Prepared as M40.

Create the best possible Lantern Leaf transcription using **only** the accepted 0.1 language.

Goals:

- prove what the outer shell can already express;
- force every approximation into the open;
- avoid schema or renderer changes;
- produce semantic, ASCII, concept, egui, expectation, and ViewWitness evidence from the same baseline;
- use human comparison against the archived concept art to identify the highest-value representational failures.

Expected result: recognizable architecture, visibly under-furnished interior.

## Stage 1 — nested semantic furnishing

### Promotion: M41

M40 human QA earned this stage. The fake reader-toolbar region, squeezed chapter status, flattened TTS command surface, and vertically stacked `Voice` label are concrete evidence that ViewWright needs region-local structure and allocation below the major-region layer.

M41 is therefore promoted narrowly as **nested furnishing layout**. It adds structural furnishing trees and local slot allocation using existing element kinds. Semantic control families remain Stage 2 work.


Primary pressure:

- toolbar → command clusters;
- inspector → settings groups → controls;
- TTS surface → media summary + transport + progress + auxiliary controls.

Current failure mode: local groups must be flattened or promoted into fake major regions.

Target capability:

- semantic grouping below the region level;
- local row/column/wrapping relationships;
- local gap/alignment/justification where justified;
- fixed/grow sizing at the furnishing level;
- stable authored IDs and accessibility ownership;
- no generic DOM/CSS container language.

This is the likely first real schema expansion after M40.

## Stage 2 — semantic control families and representative state

Pressure:

- select/choice;
- tabs/segmented choice;
- toggle;
- scalar/range adjustment;
- color choice;
- progress/seek;
- playback/transport state.

Target:

- semantic controls above toolkit widget identity;
- typed representative fixture state;
- useful egui realization;
- concept projection of control meaning and current preview state;
- application execution remains host-owned.

## Stage 3 — reader/document presentation

Pressure:

- chapter eyebrow;
- heading;
- paragraph;
- quote;
- stable block identity;
- spoken sentence/range highlight;
- literary typography;
- column width, margins, line height, paragraph spacing, alignment.

Target:

- richer document block semantics;
- representative document-range state;
- separation between application UI typography and reading typography;
- no EPUB parser or general rich-text engine.

## Stage 4 — richer visual and asset authorship

Pressure:

- UI vs reading text colors;
- faint/divider/highlight/accent-soft roles;
- icons;
- book cover/media identity;
- selective component presentation;
- possibly elevation/shadow only if still needed after structure improves.

Target:

- enough visual vocabulary to reproduce the concept's durable visual hierarchy;
- avoid arbitrary per-widget style bags.

## Stage 5 — authored responsive variants

This stage fulfills the existing mini-CSS-like ambition.

Required north-star states:

- `wide`;
- `compact`;
- `narrow`.

Target:

- root viewport width/height conditions first;
- deterministic rule selection;
- named variants;
- constrained overrides or alternate topology;
- deliberate region size/order/move/collapse/hide semantics;
- exact breakpoint tests;
- stable semantic identity across states where objects survive;
- all variants validated before runtime;
- selection occurs before backend LayoutPlan;
- live resize QA across exact boundaries.

Not allowed:

- CSS cascade/specificity;
- arbitrary media-query expressions;
- device sniffing;
- backend-only responsive heuristics.

Responsive authoring is required for v0.2 completion.

## Stage 6 — concept dossier and round-trip

Expand the deterministic concept projection so one ViewWright source can produce a rich design dossier containing:

- hierarchy;
- nested furnishing;
- representative state;
- document semantics;
- palette and typography;
- assets/media references;
- responsive states and viewport-specific intent.

The dossier should be suitable input to an external image-generation tool or human designer.

Direct image API integration is optional and not required for the core milestone.

The authoritative loop remains:

```text
ViewWright TOML
    -> resolved design
    -> concept dossier
    -> optional concept image
    -> human/agent visual judgment
    -> explicit ViewWright edits
```

## Stage 7 — north-star convergence

v0.2 converges when the Lantern Leaf north-star design can be authored without major semantic cheating.

Required review:

- wide/compact/narrow previews;
- concept-dossier review;
- concept-art comparison;
- semantic/accessibility identity audit;
- expectation vs real ViewWitness verification at representative viewports.

## What is not pre-authorized

This trajectory does not mean every stage is already a numbered milestone.

After M40, each next milestone should be promoted only when the baseline or previous stage produces concrete pressure evidence.

The trajectory is committed; exact syntax remains pressure-driven.
