# M30 — Canonical Audit

Accepted fixture-bearing sources were reviewed against root-reachable element ownership.

Representative accepted sources include:

- `examples/project-browser.toml`
- `specimens/reader-workspace.toml`
- `specimens/reader-workspace-m5.toml`
- `specimens/reader-workspace-visual.toml`
- `specimens/reader-workspace-visual-m5.toml`
- `specimens/reader-workspace-visual-m7.toml`
- `specimens/reader-workspace-visual-crushed.toml`
- `specimens/dependency-workbench.toml`
- `specimens/density-pressure-comfortable.toml`
- `specimens/density-pressure-dense.toml`

Their fixture content targets semantic elements in regions structurally placed under each source's `screen.root`. Representative targets include project/navigation collections and inspector properties, Reader outline/document/settings/transport status and commands, Dependency Workbench collections/properties/status, and density-pressure collections/properties/status.

No accepted canonical fixture content target was found to rely on an element whose owning region is outside the root composition tree.

No canonical source migration, renderer change, layout change, projection change, or visible-output change is expected for M30.

Implementation should additionally encode this audit as canonical regression coverage. If implementation discovers a contradictory accepted source, stop and report it rather than silently editing canonical fixtures.