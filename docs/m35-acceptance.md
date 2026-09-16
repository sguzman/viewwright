# M35 — Acceptance

M35 is accepted when all of the following hold:

- one typed comparator consumes `ViewWrightExpectation` plus model-only ViewWitness `Witness`;
- ViewWitness dependency is default-features-disabled and pinned to audited revision `f1930ab2a70175c46d12dd1e61501c3b4ae09408`;
- no ViewWitness repository changes are made;
- exact logical viewport width/height are compared;
- expected screen author ID presence is compared by exact `NodeIdentity.author_id` only;
- expected region author IDs are compared by exact author ID only;
- expected semantic element author IDs are compared by exact author ID only;
- missing expected identity is reported as mismatch;
- duplicate observed matches for one expected author ID are reported as ambiguity rather than silently chosen;
- extra observed author IDs are ignored in M35;
- exact region bounds are compared where observed bounds exist;
- missing observed region bounds are represented as evidence gaps, not fabricated mismatches;
- element ownership is compared through observed parent author identity against expected `region_author_id`;
- missing parent-author evidence is an evidence gap;
- contradictory explicit parent author identity is a mismatch;
- element bounds are not compared;
- role/importance/overflow/label/action/dominant/fixture/runtime facts are not compared;
- no tolerance, fuzzy matching, scoring, screenshot heuristics, or fallback identity is added;
- comparison results are deterministic for the same two inputs;
- invalid ViewWitness structure/version is rejected explicitly rather than compared silently;
- M0–M34 regressions remain green;
- README records M34 accepted and M35 implementation complete / Director audit pending.

No human screenshot QA is required because M35 has no visible UI delta.