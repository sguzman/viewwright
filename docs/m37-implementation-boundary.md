# M37 — Implementation Boundary

M37 is a toolkit compatibility and direct-adapter proof milestone.

## In scope

- align ViewWright workspace `egui` and `eframe` to 0.36.2;
- update `Cargo.lock`;
- mechanically adapt ViewWright code to required egui/eframe API changes;
- add a test-only pinned ViewWitness `egui` dependency at the narrowest crate boundary;
- render a real AccessKit-enabled ViewWright frame in a test;
- pass its `egui::FullOutput` directly to `viewwitness::witness_from_egui_output`;
- validate the resulting Witness and representative M34 author identities;
- run existing regression tests;
- perform representative human visual QA because a toolkit upgrade can alter rendering defaults;
- README milestone bookkeeping.

## Out of scope

- ViewWitness repository changes;
- a hand-written AccessKit-to-Witness translator in ViewWright;
- M35 comparator changes;
- comparing the real Witness against M33 expectation output;
- ViewWitness observer/network/server integration;
- paint/raster capture;
- new preview controls;
- new authoring syntax;
- layout redesign;
- responsive layout;
- M38.

## Runtime boundary

The ViewWitness egui adapter is test/dev proof only in M37. Do not add recurring capture work to the preview render loop.

## Stop condition

If alignment requires changing accepted ViewWright semantics, rewriting M34 author IDs, weakening M35, or modifying ViewWitness to accept ViewWright output, stop and report the architectural conflict rather than widening M37.
