# M36 — Stop Condition

M36 stops when the observable semantic author-ID namespace is unambiguous at source-validation time and all accepted bridge behavior remains unchanged.

Stop after proving:

- screen/region collisions fail;
- screen/element collisions fail;
- unused region/element collisions fail;
- screen/composition equality still passes;
- screen/fixture equality still passes;
- canonical sources pass unchanged;
- M33/M34/M35 regressions remain green.

Do not continue into:

- generated/prefixed bridge IDs;
- a general ID type system;
- ViewWitness mutation;
- comparator redesign;
- live capture integration;
- tolerance/fuzzy matching;
- responsive layout;
- M37.

If source validation alone cannot preserve the exact M33→M34→M35 identity chain, stop and report the architectural conflict rather than widening scope.
