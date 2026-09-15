# M24 — Implementation Boundary

Expected implementation is model validation only.

Validate all authored spacing/corner/color token names before successful resolution. Use deterministic ordering when iterating map-backed token families. Keep valid key strings untouched.

Do not modify egui, layout, concept/ASCII projections, token reference syntax, token value semantics, or resolved visual structures. Do not introduce a generic token registry.