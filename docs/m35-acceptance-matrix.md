# M35 — Acceptance Matrix

| Pressure | Required evidence | Accepted result |
| --- | --- | --- |
| Valid exact match | expectation and witness agree on all M35-comparable facts | zero mismatches, zero evidence gaps, `is_exact_match() == true` |
| Viewport width differs | witness width differs from expectation | typed viewport-width mismatch |
| Viewport height differs | witness height differs from expectation | typed viewport-height mismatch |
| Expected identity absent | no observed node has exact expected `author_id` | typed missing-author-ID mismatch |
| Expected identity duplicated | more than one observed node has exact expected `author_id` | typed ambiguous-author-ID mismatch; comparator chooses none |
| Extra observed author ID | witness contains authored identity not present in expectation | ignored in M35 |
| Region bounds match | uniquely matched region has equal observed logical bounds | no geometry finding |
| Region bounds differ | uniquely matched region has contradictory bounds | typed region-bounds mismatch |
| Region bounds absent | uniquely matched region has `bounds == None` | typed evidence gap, not geometry mismatch |
| Element owner matches | uniquely matched element's parent resolves to node whose `author_id` equals expected `region_author_id` | no ownership finding |
| Element owner contradicts | parent resolves and has a different explicit `author_id` | typed owner mismatch |
| Parent missing | matched element has no parent | typed ownership evidence gap |
| Parent node unavailable | element names parent node ID not present in usable evidence | invalid witness should be rejected by validation; comparator must not guess |
| Parent author ID absent | parent exists but lacks explicit author identity | typed ownership evidence gap |
| Unsupported witness version | `viewwitness_version` is not the audited supported version | comparator error; no comparison report |
| Invalid witness structure | `Witness::validation_issues()` is non-empty | comparator error containing/reporting validation issues |
| Element geometry | witness contains element bounds | ignored; M33 has no normative element geometry |
| Role/label/action/etc. differs | non-M35 facts differ | ignored in M35 |
| Repeated comparison | same expectation and same witness compared twice | structurally identical/deterministically ordered report |

## Required pressure fixtures/tests

Automated tests must cover at least:

1. exact successful comparison;
2. width and height mismatch;
3. missing screen/region/element identities;
4. duplicate observed author identity;
5. extra observed author identity ignored;
6. exact region geometry success;
7. region geometry contradiction;
8. missing region bounds evidence gap;
9. element ownership success;
10. explicit wrong-region ownership mismatch;
11. absent parent/parent-author evidence gap;
12. unsupported ViewWitness version error;
13. non-empty ViewWitness validation issues error;
14. deterministic result ordering;
15. confirmation no element geometry or deferred semantic fields enter comparison.