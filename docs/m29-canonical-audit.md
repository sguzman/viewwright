# M29 — Canonical Audit

Accepted canonical dominant targets were reviewed against their root composition trees.

- Project Browser: `dominant = "projects"`; `projects` is a direct child of root `workspace`.
- Reader Workspace variants: `dominant = "reader"`; `reader` is a child of nested `reading_body`, which is a child of root `workspace`.
- Dependency Workbench: `dominant = "packages"`; `packages` is a child of nested `body`, which is a child of root `workspace`.
- Density-pressure comfortable/dense: `dominant = "content"`; `content` is a direct child of root `workspace`.

No accepted canonical dominant target is unreachable. No canonical source migration, renderer change, layout change, projection change, or visible-output change is expected for M29.

If implementation discovers a contradictory accepted source, stop and report it rather than silently modifying canonical source.