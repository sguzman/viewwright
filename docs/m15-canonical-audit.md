# M15 — Canonical Audit

The accepted canonical pressure surface was audited for `design.dominant`.

Observed accepted values:

- `examples/project-browser.toml` → `projects`
- `specimens/reader-workspace.toml` → `reader`
- `specimens/reader-workspace-visual.toml` → `reader`
- `specimens/dependency-workbench.toml` → `packages`
- `specimens/density-pressure-comfortable.toml` → `content`
- `specimens/density-pressure-dense.toml` → `content`

Each of those identifiers names a region.

No canonical source migration is required.

The current resolver already accepts element ids as valid dominant targets. M15 preserves that source contract and should cover it with a focused synthetic/model test rather than inventing a canonical specimen solely to exercise it.

If implementation discovers canonical dominant targets outside region/element semantics, stop and report rather than widening M15.
