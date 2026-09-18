# M42 — Implementation Boundary

## In scope

- choice/boolean/scalar element kinds;
- choice/scalar typed configuration;
- typed fixture state for all three;
- fixture-scoped renderer-local value state;
- typed value-change interaction reports;
- semantic/debug/ASCII/concept projection updates;
- fixture-aware concept control-state projection;
- egui realization;
- expectation 0.2 kind vocabulary;
- new Lantern Leaf M42 control specimen;
- focused validation/interaction/AccessKit/exact-verification tests;
- preview human QA.

## Expected crates

Likely:

- viewwright-model;
- viewwright-ascii;
- viewwright-concept;
- viewwright-egui;
- viewwright-expectation;
- viewwright-preview.

viewwright-layout should require no semantic change.

viewwright-compare should not change algorithmically; only tests/docs if necessary for expectation 0.2.

## Out of scope

- color picker/color-valued control;
- tabs and conditional content;
- progress/seek/waveform;
- media summary;
- icons/assets;
- rich document blocks/ranges;
- font-family rendering itself;
- application settings execution;
- persistent state/data binding;
- synchronizing the same application value across two authored controls;
- richer visual roles;
- responsive rules;
- ViewWitness repository changes;
- M43.
