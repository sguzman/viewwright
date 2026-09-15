# M14 — Canonical Audit

The initial M14 canonical audit was incorrect: `examples/project-browser.toml` still contains three canonical elements whose visible labels are currently synthesized by the resolver from their ids.

Confirmed omissions:

- `navigation_items` → current synthesized visible text `navigation items`
- `project_collection` → current synthesized visible text `project collection`
- `project_inspector` → current synthesized visible text `project inspector`

Other audited canonical representatives already author explicit element labels:

- `specimens/reader-workspace.toml`
- `specimens/reader-workspace-visual.toml`
- `specimens/dependency-workbench.toml`
- `specimens/density-pressure-comfortable.toml`
- `specimens/density-pressure-dense.toml`

## Authorized canonical migration

M14 is explicitly authorized to add these three labels to `examples/project-browser.toml`:

```toml
label = "navigation items"
label = "project collection"
label = "project inspector"
```

The exact lowercase strings are intentional: they preserve the already-accepted visible output of the old `id.replace('_', " ")` fallback while moving authority into canonical authored TOML.

This is a fidelity migration, not a copy redesign. Do not improve, title-case, rename, or otherwise reinterpret these labels during M14.

Any additional unlabeled canonical accepted element discovered during implementation is still a stop condition and must be reported rather than silently edited.

Test-only snippets relying on the old fallback may be given explicit meaningful labels as needed to preserve the behavior under test.

This corrected audit preserves M14's zero-visible-delta requirement while eliminating the id-derived authority leak.