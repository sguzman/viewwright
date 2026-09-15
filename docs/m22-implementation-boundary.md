# M22 — Implementation Boundary

M22 should be a model-validation correction only.

Expected implementation surface:

- `crates/viewwright-model/src/lib.rs` validation and focused tests;
- README milestone bookkeeping.

Do not change renderer, layout, fixture payload families, source syntax, resolved fixture shape, or preview behavior for valid fixtures.

Use whitespace only for blankness detection. Do not trim stored values.

If implementation appears to require state enums, transition semantics, fixture selection redesign, general copy validation, or renderer changes, stop and report the pressure instead of broadening M22.