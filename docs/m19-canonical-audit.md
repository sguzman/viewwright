# M19 — Canonical Audit

Director audit of accepted visual sources found no malformed color-token literals and no migration pressure.

## Canonical visual sources

- `specimens/reader-workspace-visual.toml` authors the accepted Reader palette; every color token is a valid six-digit hex literal.
- `specimens/dependency-workbench.toml` authors its light palette; every color token is a valid six-digit hex literal.
- `specimens/reader-workspace-visual-m7.toml` retains the accepted Reader palette for command-affordance pressure; every color token is valid.

## Canonical non-visual sources

Project Browser, structural Reader, and the density pressure pair do not rely on malformed color-token data; sources without `[tokens.color]` remain unaffected.

## Expected migration

None.

M19 should change only rejection of malformed authored color tokens that previously escaped validation because they were unreferenced. Valid canonical resolution and visual output should remain byte/semantics-equivalent where currently tested.