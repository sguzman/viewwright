# M22 — Canonical Audit

Director audit found no accepted fixture with empty or whitespace-only `state`.

Observed canonical fixture-state examples include:

- Dependency Workbench: `ready`, `attention`;
- Reader variants: meaningful loaded/empty state names such as `document_loaded` and `no_document`;
- Project Browser and density pressure specimens: nonblank populated-state descriptions.

## Expected migration

None.

M22 should only reject malformed present-but-blank fixture state that previously survived resolution. Valid canonical source, semantic/debug output, layout, and renderer behavior should remain unchanged.