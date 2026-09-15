# M16 — Implementation Boundary

Implementation should be validation-led and model-local.

Expected work:

- inspect resolved/authored composition child relationships after child references and root are known;
- track parent ownership by child semantic id/category;
- reject duplicate sibling references;
- reject multiple parents for regions;
- reject multiple parents for nested compositions;
- reject any parent relationship targeting the root composition;
- preserve existing cycle detection and diagnostics.

Do not redesign `LayoutPlan`, renderer traversal, source syntax, or composition data structures merely to enforce this invariant.

Do not add reachability validation, automatic deduplication, implicit cloning, or graph semantics.