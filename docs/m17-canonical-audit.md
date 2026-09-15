# M17 Canonical Audit — Structural Identifier Fidelity

The accepted canonical pressure was inspected before M17 implementation.

Audited sources:

- `examples/project-browser.toml`
- `specimens/reader-workspace.toml`
- `specimens/reader-workspace-visual.toml`
- `specimens/dependency-workbench.toml`
- `specimens/density-pressure-comfortable.toml`
- `specimens/density-pressure-dense.toml`

Result:

- screen IDs are nonblank;
- region IDs are nonblank;
- composition IDs are nonblank;
- element IDs are nonblank;
- fixture IDs are nonblank;
- authored collection item IDs are nonblank;
- authored tree node IDs are nonblank;
- canonical root references name nonblank composition IDs.

Therefore M17 requires no canonical source migration and should produce no visible delta.

If implementation discovers a canonical blank/whitespace-only ID that this audit missed, stop and report the contradiction rather than silently renaming it.