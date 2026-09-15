# M18 Canonical Audit

Canonical accepted sources were audited for composition topology before implementation pressure is handed to Codex.

- `examples/project-browser.toml`: one root composition; acyclic.
- `specimens/reader-workspace.toml`: `workspace -> reading_body`; acyclic.
- `specimens/reader-workspace-visual.toml`: `workspace -> reading_body`; acyclic.
- `specimens/dependency-workbench.toml`: `workspace -> body`; acyclic.
- `specimens/density-pressure-comfortable.toml`: one root composition; acyclic.
- `specimens/density-pressure-dense.toml`: one root composition; acyclic.

No canonical source contains an unreachable composition cycle. No canonical migration is expected for M18.

M18 pressure is synthetic regression pressure against a validator-scope hole, not a request to rewrite accepted specimens.
