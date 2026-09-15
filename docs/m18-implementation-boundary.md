# M18 Implementation Boundary

Implement only global composition-cycle validation.

## In scope

- reuse or minimally adapt the existing composition DFS/cycle detector;
- ensure traversal covers every authored/resolved composition, not only `screen.root` descendants;
- retain deterministic source-order diagnostics;
- add focused model tests;
- preserve all accepted M0–M17 behavior.

## Out of scope

- requiring reachability from root;
- deleting or warning on unused declarations;
- new composition syntax;
- parent fields or parent pointers in source;
- generic graph libraries or graph APIs;
- scene-graph semantics;
- reusable component instances;
- layout or egui changes;
- projection redesign;
- M19 work.

Cycle validation belongs in parse/resolve and must add no recurring render-thread work.
