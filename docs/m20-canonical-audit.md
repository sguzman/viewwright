# M20 — Canonical Audit

Director audit found no canonical source with a blank or whitespace-only `screen.purpose`.

Accepted sources checked:

- `examples/project-browser.toml` — purpose is nonblank.
- `specimens/reader-workspace.toml` — purpose is nonblank.
- `specimens/reader-workspace-visual.toml` — purpose is nonblank.
- `specimens/reader-workspace-visual-m7.toml` — purpose is nonblank.
- `specimens/dependency-workbench.toml` — purpose is nonblank.
- `specimens/density-pressure-comfortable.toml` — purpose is nonblank.
- `specimens/density-pressure-dense.toml` — purpose is nonblank.

## Expected migration

None.

M20 should change only rejection of semantically blank required purpose text. Valid canonical purpose strings and all downstream projections should remain unchanged.
