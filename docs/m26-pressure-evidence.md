# M26 — Pressure Evidence

M3 established property-sheet fixture content as ordered `{ name, value }` representative metadata. Current resolution copies property names and values directly into `ResolvedProperty`. The egui backend renders each property name as the strong visible label beside its value.

Therefore an authored property with an empty/whitespace-only `name` survives resolution and renders as an unlabeled metadata row. That is loss of authored visible semantics, not merely cosmetic text quality.

Property values are deliberately different pressure: an empty value can truthfully represent absent/unknown metadata and is not invalidated by M26.