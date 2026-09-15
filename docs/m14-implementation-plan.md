# M14 — Implementation Plan

Implementation should remain narrow and model-led:

1. add one small label-validation helper or equivalent in `viewwright-model`;
2. validate every current element's `label` before constructing `ResolvedElement`;
3. reject missing, empty, and whitespace-only labels with element-specific diagnostics;
4. remove `id.replace('_', " ")` or any equivalent id-derived fallback;
5. keep `ResolvedElement.label: String` populated only from authored source;
6. update test-only TOML snippets that omitted labels under the old fallback;
7. add focused validation/provenance tests;
8. verify canonical specimens and all projections remain unchanged in intent;
9. run the full workspace regression suite.

No renderer redesign is expected.