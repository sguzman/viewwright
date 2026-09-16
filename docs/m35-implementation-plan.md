# M35 — Implementation Plan

## 1. Add comparator crate/module

Prefer a small workspace crate such as:

`viewwright-compare`

It should depend on:

- `viewwright-expectation`;
- `viewwitness` from GitHub revision `f1930ab2a70175c46d12dd1e61501c3b4ae09408` with `default-features = false`.

Do not enable ViewWitness egui/observer/showcase features.

## 2. Define typed result model

Expose an API conceptually equivalent to:

```text
compare(expectation: &ViewWrightExpectation, witness: &Witness)
    -> Result<ComparisonReport, ComparisonError>
```

Use typed mismatch and evidence-gap enums.

`ComparisonReport` should contain ordered collections for:

- mismatches;
- evidence gaps.

Provide `is_exact_match()` only as the conjunction that both collections are empty.

## 3. Validate witness before comparison

Before producing findings:

1. require the audited supported `viewwitness_version` (`0.1` at the pinned revision);
2. call `Witness::validation_issues()`;
3. if validation issues are non-empty, return a comparator error rather than partial findings.

Do not repair malformed witnesses.

## 4. Build exact author-ID index

Index only nodes whose `identity.author_id` is present.

Preserve all matches for an author ID so duplicates remain observable.

Do not choose the first duplicate.

Observed nodes without author ID are irrelevant to identity matching.

Extra observed author IDs are retained in the index but generate no finding unless requested by the expectation.

## 5. Compare viewport

Compare expectation width and height exactly against `witness.capture.viewport.width/height`.

Ignore `scale_factor` in M35.

Produce width/height findings in stable order.

## 6. Compare expected identity presence

Walk expectation facts deterministically:

1. screen;
2. regions in M33 expectation order;
3. elements in M33 expectation order.

For each expected author ID:

- zero matches → missing-author mismatch;
- one match → continue;
- more than one match → ambiguity mismatch and do not perform dependent geometry/ownership checks for that identity.

The mismatch should identify whether the expected object is screen, region, or element.

## 7. Compare region bounds

For every uniquely matched expected region:

- if observed `bounds` is absent, emit a region-bounds evidence gap;
- otherwise compare `x`, `y`, `width`, and `height` exactly against M33 expectation bounds;
- any difference produces one typed region-bounds mismatch carrying expected and observed rectangles.

Do not add epsilon/tolerance logic.

## 8. Compare element ownership

For every uniquely matched expected element:

1. inspect its observed `parent` witness-node ID;
2. if absent, emit ownership evidence gap;
3. resolve the parent node by witness-local `Node::id`;
4. structural missing-parent cases should already be rejected by `validation_issues()`;
5. inspect parent `identity.author_id`;
6. absent parent identity/author ID → ownership evidence gap;
7. exact expected `region_author_id` → pass;
8. different explicit author ID → owner mismatch.

Do not climb ancestors or guess through geometry/labels.

M34's semantic hierarchy is the pressure being tested.

## 9. Deterministic ordering

Findings must be deterministic for identical inputs.

Use explicit traversal/order rather than hash iteration.

Recommended finding order:

- viewport width;
- viewport height;
- screen identity;
- each expected region: identity then bounds;
- each expected element: identity then ownership.

If the implementation uses separate mismatch/evidence-gap vectors, preserve encounter order within each collection.

## 10. Tests

Use typed synthetic `Witness` values and real/typed `ViewWrightExpectation` values.

Cover the complete acceptance matrix, including:

- exact match;
- viewport mismatches;
- missing and duplicate identities;
- extra observed identities ignored;
- region exact/mismatch/no-bounds cases;
- element owner exact/wrong/missing-evidence cases;
- unsupported version;
- invalid witness validation;
- repeated deterministic comparison;
- confirmation deferred fields and element geometry do not affect M35 report.

Where useful, use a real M33 Project Browser expectation as pressure, but do not require live egui capture to unit-test the comparator.

## 11. Preserve architecture

No changes should be needed to:

- ViewWright TOML/schema/model;
- LayoutPlan;
- M33 expectation wire format;
- viewwright-egui / M34 identity projection;
- ViewWitness repository.

M35 has no intended visible delta.

## 12. Bookkeeping

Update README:

- M34 — accepted;
- M35 — implementation complete; director audit pending.

Leave the M35 issue open for Director audit.

Do not begin M36.