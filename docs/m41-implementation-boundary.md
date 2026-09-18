# M41 — Implementation Boundary

## In scope

- source/resolved furnishing model;
- validation and structural ownership;
- furnishing rectangles in `viewwright-layout`;
- semantic/debug, ASCII, and concept projection of furnishing structure;
- egui rendering of furnished regions;
- one new Lantern Leaf furnished pressure specimen;
- focused validation/layout/render/AccessKit/verification tests;
- preview registration;
- human QA.

## Expected Rust crates

Likely touched:

- `viewwright-model`;
- `viewwright-layout`;
- `viewwright-ascii`;
- `viewwright-concept`;
- `viewwright-egui`;
- `viewwright-preview`.

`viewwright-expectation` and `viewwright-compare` should normally need only regression tests, not semantic changes.

## Out of scope

Do not add:

- new semantic element kinds;
- new fixture content families;
- runtime values for future controls;
- document block schema;
- asset/icon schema;
- font-family or richer palette schema;
- responsive syntax;
- alignment/justify/cascade language beyond the fixed/grow/gap/padding model;
- horizontal overflow;
- furnishing visual surfaces;
- furnishing accessibility author IDs;
- expectation format changes;
- comparator changes;
- ViewWitness changes;
- M42.

## Existing M40 baseline

Do not rewrite `specimens/lantern-leaf-reader-baseline.toml`.

It is historical pressure evidence.

Add a separate furnished specimen so the improvement is directly comparable.

## Behavior boundary

Existing unfurnished specimens must not visually change because of M41.

If implementing furnishing requires altering legacy flat-region behavior, stop and justify the change rather than silently coupling the paths.
