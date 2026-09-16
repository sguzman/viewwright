# M32 — Canonical Audit

Accepted canonical sources were reviewed before adding region overflow authoring.

Current accepted region declarations do not author an overflow field. Their content is intentionally short enough that no accepted canonical screen currently depends on scrolling for correctness.

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
- `specimens/overlay-command-palette-pressure.toml`

M32 therefore uses omitted overflow = `clip` as the compatibility baseline and expects no canonical source migration.

The new pressure source is intentionally not pre-M32 canonical:

- `specimens/reader-overflow-pressure.toml`

It is derived from the accepted visual Reader structure but adds a long document and authors `overflow = "scroll_y"` only on region `reader`.

Layout geometry should remain identical to the equivalent Reader geometry at a given viewport size. Only content overflow behavior inside that region should differ.

If implementation requires modifying an existing accepted canonical source merely to support overflow, stop and report the contradiction.
