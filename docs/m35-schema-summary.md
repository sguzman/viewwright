# M35 — Schema Summary

M35 adds no authored TOML fields, no resolved blueprint fields, and no changes to the M33 expectation wire format.

It adds a comparator API, likely in a small `viewwright-compare` crate, consuming:

```text
ViewWrightExpectation
+
viewwitness::Witness
→
ComparisonReport
```

## Comparison report

The report must distinguish two categories:

- `mismatches`: observed evidence contradicts or fails an exact expected fact;
- `evidence_gaps`: the witness lacks enough evidence to evaluate an otherwise comparable fact.

A convenience `is_exact_match()` may return true only when both collections are empty.

## Minimum mismatch vocabulary

Typed variants should cover at least:

- viewport width/height mismatch;
- missing expected author ID;
- ambiguous/duplicate observed author ID;
- region bounds mismatch;
- element parent/owner mismatch.

## Minimum evidence-gap vocabulary

Typed variants should cover at least:

- matched region has no observed bounds;
- matched element lacks enough parent/parent-author identity evidence to evaluate ownership.

## Comparator errors

Malformed/unsupported input is not a comparison finding. Return an explicit comparator error for at least:

- unsupported `viewwitness_version`;
- non-empty `Witness::validation_issues()`.

## Exact-match domain

M35 compares only:

- viewport width/height;
- expected author-ID presence/uniqueness;
- region bounds;
- element owning-region identity.

No element bounds, semantic-role mapping, label comparison, action comparison, dominant comparison, fixture comparison, or tolerance schema is added.