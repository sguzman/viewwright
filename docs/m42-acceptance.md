# M42 — Acceptance

## Model

- choice/boolean/scalar are typed semantic element kinds.
- choice config is required only for choice.
- scalar config is required only for scalar.
- control actions use existing stable namespaced action validation.
- choice options validate cardinality, nonblank IDs/labels, and local uniqueness.
- scalar range/step/unit validate strictly.
- existing element kinds/sources remain backward compatible.

## Fixture state

- choice/boolean/scalar state is typed.
- each fixture explicitly covers every M42 control.
- choice selection must exist.
- scalar state must be finite/in range.
- duplicate or cross-kind control state is rejected.
- fixture switching/reset returns preview-local state to canonical fixture seeds.

## Projection

Semantic/debug and concept projection expose:

- semantic control family;
- choice presentation/options;
- scalar range/step/unit;
- action identity.

Add a fixture-aware concept projection that can expose representative M42 control values for one named fixture while preserving the existing structural `render(blueprint)` API.

ASCII should identify M42 control kinds/presentation without pretending to execute them.

## egui

- choice/select uses an appropriate egui finite-choice presentation.
- choice/segmented uses a compact segmented/selectable presentation.
- boolean uses a boolean control.
- scalar uses a bounded slider with authored step/unit.
- fixture values seed visible state.
- interaction changes remain local to RenderState until host logic acts.
- changed values emit a typed interaction event with element/action/value.
- command interaction behavior remains unchanged.
- no heavy work enters the render loop.

## Expectation / verification

- expectation canonical format is 0.2.
- element kinds Choice/Boolean/Scalar export truthfully.
- M35 comparison algorithm remains unchanged.
- ViewWitness remains pinned/unchanged.
- existing exact-verification cases still pass.
- new M42 Lantern Leaf specimen passes real FullOutput -> ViewWitness -> M35 exact comparison with zero gaps/mismatches.

## Lantern Leaf specimen

Add a new M42 specimen; do not mutate M40 or M41 specimens.

It should replace important fake/display-only values with honest controls, including representative examples of all three families.

At minimum:

- font family: choice/select;
- font size: scalar;
- line height: scalar;
- paragraph spacing: scalar;
- page margin: scalar;
- column width: scalar;
- text alignment: choice/segmented;
- dyslexia-friendly font: boolean;
- reading flow: choice/segmented;
- show highlights/notes/bookmarks: booleans;
- bottom Voice: choice/select;
- bottom Speed: choice/select;
- bottom Volume: scalar.

Color picker/preset-grid/progress may remain approximated or absent.

## Human QA

Human should verify:

- inspector now reads as actual settings rather than property-sheet prose;
- choice/select controls show the intended fixture values;
- segmented controls read clearly;
- booleans read as true/false controls;
- scalar controls are legible and not clipped;
- Voice/Speed/Volume are value controls, not fake commands;
- manipulating at least one choice, one boolean, and one scalar visibly changes preview-local state;
- preview reports typed action/value evidence;
- reader remains dominant and M41 furnishing stays visually coherent.

No claim of final visual parity.

## Regression

M0–M41 remains green.

No progress/seek, rich document, visual-role, responsive, or M43 work.
