# M30 — Implementation Boundary

Keep M30 as a model-resolution validation change.

Allowed implementation work:

- reuse or locally expose the M29 root-reachable region set;
- after resolving a fixture content record's element reference, validate that the element's owning region is root-reachable when the root is valid;
- add focused model regressions;
- update milestone bookkeeping.

Do not:

- require every declaration to be root-reachable;
- require every reachable element to have fixture content;
- delete or warn on unused declarations;
- change fixture payload syntax or resolved payload variants;
- alter payload-family compatibility semantics;
- alter M18 global cycle validation;
- alter M29 dominant semantics;
- change renderer or layout behavior;
- introduce a generic graph framework;
- start M31.

When `screen.root` is missing or invalid, preserve existing root errors without dependent fixture-content reachability diagnostics.

All work remains outside the render loop.