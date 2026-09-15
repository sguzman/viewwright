# M16 — Canonical Audit

Accepted composition pressure was checked for unique structural placement.

- Project Browser: one root split; `navigation`, `projects`, and `inspector` each appear once.
- Reader Workspace: root `workspace` owns `app_commands`, nested `reading_body`, and `transport`; `reading_body` uniquely owns `library`, `reader`, and `inspector`.
- Visual Reader follows the same nested-tree topology.
- Dependency Workbench: root `workspace` owns `commands`, nested `body`, and `status`; `body` uniquely owns `filters`, `packages`, and `inspector`.
- Comfortable/Dense density pressure: one root split uniquely owns `controls` and `content`.

No accepted canonical specimen requires duplicate siblings, region multiple parents, nested-composition multiple parents, or root-as-child semantics.

No canonical source migration is expected for M16.

If implementation discovers an accepted canonical multiple-parent relationship, stop and report it rather than silently changing the specimen or widening graph semantics.