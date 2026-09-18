# M42 — Expectation Format 0.2

M33 introduced ViewWrightExpectation 0.1 with a closed element-kind vocabulary.

M42 adds three normative semantic element kinds. Because element kind is an exported field, silently adding new wire values under the old format version would make the version misleading.

Therefore canonical expectation export advances to 0.2.

## What changes

Only normative vocabulary:

- `ElementKind::Choice`;
- `ElementKind::Boolean`;
- `ElementKind::Scalar`;
- top-level canonical version becomes `0.2`.

## What does not change

No new fields are required.

Existing fields retain meaning:

- viewport;
- screen ID;
- dominant;
- regions;
- region bounds;
- element owner;
- importance;
- labels;
- actions.

## Comparison

M35 never compares element kind or representative fixture value.

Its exact shared-evidence algorithm therefore remains valid for typed 0.2 expectations without semantic weakening.

Do not add kind comparison merely because 0.2 exists.

## Historical handling

M33 authority and historical 0.1 documents remain unchanged.

The Rust expectation type may retain a V0_1 enum variant when useful, but new canonical `build_expectation` results are V0_2.

Existing M38/M40/M41 end-to-end tests should continue to pass with newly built 0.2 expectations.
