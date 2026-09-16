# M35 — Stop Condition

Stop M35 when ViewWright can compare one `ViewWrightExpectation` against one model-only ViewWitness `Witness` deterministically and truthfully across the exact shared evidence surface.

The milestone stops when all of these hold:

- ViewWitness is consumed only as a pinned, default-features-disabled model dependency at revision `f1930ab2a70175c46d12dd1e61501c3b4ae09408`;
- unsupported `viewwitness_version` and non-empty `validation_issues()` are explicit comparator errors;
- logical viewport width/height are compared exactly;
- expected screen, region, and semantic-element identities are matched only by exact `NodeIdentity.author_id`;
- missing expected identity is a mismatch;
- duplicate observed matches for one expected author ID are an ambiguity mismatch;
- extra observed identities are ignored;
- matched region bounds are compared exactly when observed bounds exist;
- absent observed region bounds are evidence gaps;
- matched element ownership is checked through the element node's parent node and that parent's explicit author ID;
- contradictory explicit parent author identity is a mismatch;
- missing parent/parent-author evidence is an evidence gap;
- result ordering is deterministic;
- `is_exact_match()` is true only when both mismatch and evidence-gap collections are empty;
- M0–M34 regressions remain green;
- no renderer, AccessKit, authored TOML, LayoutPlan, M33 expectation schema, or ViewWitness repository behavior changes are required.

Do not continue in M35 into:

- tolerances;
- fuzzy/fallback matching;
- scoring;
- label/role/action/dominant comparison;
- element geometry;
- screenshot or paint comparison;
- live capture orchestration;
- CI gating;
- responsive layout;
- M36.