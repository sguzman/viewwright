# M31 — Canonical Audit

Accepted canonical sources were reviewed for composition-kind and axis compatibility before re-admitting overlay.

Current accepted canonical screens use only the already-supported linear kinds:

- `split`
- `row`
- `column`

No accepted canonical source depends on `stack` or `overlay` today.

Representative accepted sources include:

- `examples/project-browser.toml`
- `specimens/reader-workspace.toml`
- `specimens/reader-workspace-m5.toml`
- `specimens/reader-workspace-visual.toml`
- `specimens/reader-workspace-visual-m5.toml`
- `specimens/reader-workspace-visual-m7.toml`
- `specimens/dependency-workbench.toml`
- `specimens/density-pressure-comfortable.toml`
- `specimens/density-pressure-dense.toml`

They should require no migration and should retain identical linear layout geometry and rendering.

The new pressure source is intentionally not pre-M31 canonical:

- `specimens/overlay-command-palette-pressure.toml`

It uses `kind = "overlay"` with exactly two children: base composition `workspace` followed by fixed-size floating region `palette_surface`. It authors no overlay axis or gap, and the floating region authors both fixed width and height with no positive grow.

Its existing M29 dominant target and M30 fixture-content targets are structurally reachable under the intended overlay tree.

If implementation requires modifying an existing accepted canonical source to accommodate overlay, stop and report the contradiction. M31 should add a new topology without changing existing linear semantics.
