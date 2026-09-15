# M14 — Implementation Plan

Implementation should remain narrow and model-led:

1. add one small label-validation helper or equivalent in `viewwright-model`;
2. validate every current element's `label` before constructing `ResolvedElement`;
3. reject missing, empty, and whitespace-only labels with element-specific diagnostics;
4. remove `id.replace('_', " ")` or any equivalent id-derived fallback;
5. keep `ResolvedElement.label: String` populated only from authored source;
6. perform the explicitly authorized Project Browser migration from `docs/m14-canonical-audit.md`: add `navigation items`, `project collection`, and `project inspector` as authored labels matching the old visible fallback exactly;
7. update test-only TOML snippets that omitted labels under the old fallback;
8. add focused validation/provenance tests;
9. verify canonical specimens and all projections preserve their accepted visible/semantic intent;
10. run the full workspace regression suite.

No renderer redesign is expected. The three Project Browser label additions are a fidelity migration only; do not rewrite their copy during M14.