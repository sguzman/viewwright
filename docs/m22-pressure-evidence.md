# M22 — Pressure Evidence

ViewWright has treated fixtures as authored representative states since M0 and M3. Their state description is not inferred renderer behavior: it is a required source field carried into `ResolvedFixture` and semantic/debug inspection.

Current source/model shape permits:

```toml
[[fixture]]
id = "example"
state = "   "
```

to resolve successfully.

That creates a canonical representative fixture whose required state description contains no semantic content.

Accepted specimens demonstrate the intended use with meaningful states such as `ready`, `attention`, `document_loaded`, `no_document`, and populated-state descriptions.

M22 closes only this present-but-blank state gap.