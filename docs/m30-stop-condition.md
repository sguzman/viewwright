# M30 — Stop Condition

Stop M30 once fixture content can no longer target structurally unreachable elements without a validation error, while all of the following remain true:

- reachable direct and nested targets still resolve;
- missing element and payload-kind diagnostics remain intact;
- invalid/missing roots do not cause reachability cascade noise;
- fixtures may omit content freely;
- unrelated unused declarations remain legal;
- M18 global cycle validation remains global;
- M29 dominant behavior remains unchanged;
- accepted canonical sources require no migration;
- renderer/layout/projections are unchanged for valid sources.

Do not continue into fixture coverage, dead-code analysis, runtime data binding, or M31.