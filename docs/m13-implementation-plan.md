# M13 — Implementation Plan

Implementation should remain small and compiler-driven:

1. introduce model enums for composition kind and axis;
2. add source-string validation/resolution helpers;
3. encode row/column implied-axis compatibility rules;
4. preserve split's existing omitted-axis behavior;
5. change `ResolvedComposition.kind` and `.axis` to typed values;
6. update semantic/debug and concept projections;
7. update M4 layout to branch on typed axis/kind;
8. migrate direct `ResolvedComposition` test construction;
9. add focused acceptance tests;
10. run the full workspace regression suite.

No renderer redesign or layout-feature expansion is expected.