# M12 — Pressure Evidence

The current source and resolved models both treat `region.role` as arbitrary text:

- `RegionSource.role: String`
- `ResolvedRegion.role: String`

Resolution copies the authored value directly without validation.

At the same time, accepted behavior already depends on role semantics:

- egui checks the literal role `commands` to choose horizontal command-region rendering;
- M10 explicitly preserves region role in semantic/debug and concept inspection;
- accepted canonical specimens repeatedly use a stable role vocabulary.

Observed accepted role spellings are:

- `commands`
- `controls`
- `navigation`
- `primary_content`
- `inspector`
- `status`

A typo therefore creates silent semantic drift. M12 closes that gap by making the existing ontology typed and validated.
