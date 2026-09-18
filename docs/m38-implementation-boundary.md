# M38 — Implementation Boundary

M38 is an in-process integration-proof milestone.

## In scope

- add the narrow dev/test dependencies needed for one test boundary to call:
  - `viewwright_expectation::build_expectation`;
  - `viewwright_egui::show`;
  - `viewwitness::witness_from_egui_output`;
  - `viewwright_compare::compare`;
- add a focused integration test at the existing `viewwright-egui` test boundary or an equivalently narrow test-only location;
- use real AccessKit-enabled egui `FullOutput`;
- use the pinned ViewWitness egui adapter directly;
- assert exact M35 match for Project Browser, M31 overlay pressure, and M32 overflow pressure;
- produce useful diagnostics when a case fails;
- README milestone bookkeeping.

## Preferred dependency shape

`viewwright-egui` already owns the M37 test-only ViewWitness egui dependency.

Prefer adding dev dependencies on:

- `viewwright-expectation`;
- `viewwright-compare`.

Do not move ViewWitness egui into normal renderer dependencies.

Do not make `viewwright-compare` depend on egui.

## Out of scope

- production verification API;
- new crate unless the existing test boundary proves structurally impossible;
- recurring verification in the preview render loop;
- CLI verification command;
- network capture;
- observer server;
- external ViewWitness process;
- screenshot/paint comparison;
- comparator changes;
- expectation changes;
- renderer behavior changes;
- responsive layout;
- CI gating;
- M39.

## Runtime boundary

M38 runs only when explicitly invoked by tests.

It must not add work to ordinary preview frames or application runtime.

## Stop rule

If exact verification cannot pass without modifying M33 expectation meaning, M34 identity/bounds semantics, M35 comparison semantics, or ViewWitness behavior, stop and report the real incompatibility instead of widening M38.
