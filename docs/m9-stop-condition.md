# M9 — Stop Condition

M9 stops when existing authored screen density is typed, inspectable, and produces a bounded micro-layout distinction in egui.

Stop when:

1. `comfortable` and `dense` are typed resolved density values;
2. unknown density values are rejected;
3. omitted density resolves to Comfortable;
4. semantic/debug output exposes density;
5. concept output exposes density;
6. egui uses a centralized micro-density policy rather than ignoring the field;
7. the Comfortable and Dense A/B pressure specimens are observably different in local rhythm;
8. the A/B pair has identical M4 layout plans at the same viewport;
9. density does not change type scale, palette, fixtures, actions, or collection presentation;
10. Project Browser remains comfortable and legible;
11. visual Reader remains comfortable and preserves accepted M6 visuals;
12. Dependency Workbench remains dense and legible;
13. accepted M0–M8 behavior remains intact.

Do not continue into:

- per-region density;
- per-element density;
- additional density modes;
- global blueprint scaling;
- font-size scaling;
- responsive density switching;
- generalized spacing authoring;
- M4 layout changes;
- ViewWitness integration.

M9 is complete once `screen.density` stops being decorative metadata and becomes a truthful, bounded projection semantic.
