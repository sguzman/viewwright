# M7 — Stop Condition

M7 stops when stable command affordance identity survives authoring, resolution, fixture preview, and renderer activation reporting without ViewWright taking ownership of application behavior.

Stop when:

1. command elements author validated stable action identifiers;
2. non-command action misuse is rejected;
3. static fixture command enabled/disabled state resolves cleanly;
4. disabled commands render disabled and emit no activation;
5. enabled command clicks report semantic element id + action id;
6. preview-host chrome can display the emitted action without contaminating the authored screen;
7. canonical accepted command elements are migrated away from label-only meaning;
8. semantic/debug output can inspect the new affordance contract;
9. M0–M6 tests and behavior remain intact.

Do not continue into:

- executing actions;
- callbacks or handlers in TOML;
- action parameters;
- generalized event routing;
- runtime state graphs;
- keyboard shortcuts;
- navigation systems;
- collection/tree interaction models;
- ViewWitness integration.

The milestone is complete once ViewWright can say **what action an authored command affords** and report that the affordance was activated. What the application does next remains outside ViewWright.