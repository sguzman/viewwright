# M14 — Canonical Audit

The accepted canonical pressure surface already authors explicit element labels.

Audited representatives:

- `examples/project-browser.toml`
- `specimens/reader-workspace.toml`
- `specimens/reader-workspace-visual.toml`
- `specimens/dependency-workbench.toml`
- `specimens/density-pressure-comfortable.toml`
- `specimens/density-pressure-dense.toml`

M14 therefore requires no canonical source migration for accepted screens.

Any unlabeled elements found during implementation should be classified before editing:

- canonical accepted specimen: stop and report the contradiction;
- regression/test-only snippet relying on the old fallback: add an explicit test label;
- dead or obsolete fixture: do not silently widen scope; report if cleanup is nontrivial.

This protects M14's zero-visible-delta claim.