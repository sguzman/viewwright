# M44 — Implementation Boundary

## In scope

- optional responsive source/resolved model;
- deterministic width interval validation/selection;
- alternate active composition roots;
- sparse region width/height/grow/furnishing overrides;
- variant-local furnishing ownership/coverage validation;
- responsive evolution of dominant/fixture reachability;
- responsive-aware LayoutPlan;
- renderer traversal from active plan/root;
- viewport-aware ASCII/concept projections;
- responsive-aware expectation export using existing 0.2 schema;
- one new Lantern Leaf responsive specimen;
- preview active-variant visibility for QA;
- wide/compact/narrow exact-verification tests;
- live resize human QA.

## Likely crates

- viewwright-model;
- viewwright-layout;
- viewwright-ascii;
- viewwright-concept;
- viewwright-egui;
- viewwright-expectation;
- viewwright-preview.

viewwright-compare should not change algorithmically.

ViewWitness should not change.

## Out of scope

- new semantic element kinds;
- responsive visual/theme values;
- responsive fixture payload values;
- responsive screen density;
- height/aspect/container conditions;
- CSS cascade/specificity;
- auto-responsive heuristics;
- animations;
- drawers/modal navigation runtime;
- Stage 4 visual-role/font/asset expansion;
- M45.

## Existing specimens

Do not retrofit responsive blocks into historical specimens.

M44 adds a separate north-star responsive specimen.
