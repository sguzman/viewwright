# M26 — Acceptance

M26 is accepted when:

- `PropertySource.name = ""` is rejected;
- whitespace-only property names are rejected using Rust `trim()` semantics;
- valid property names are preserved exactly, including intentional surrounding whitespace, punctuation, Unicode, and case;
- property values remain unconstrained by M26 and may be empty;
- duplicate property names remain legal;
- authored property order remains unchanged;
- omitted/empty property lists remain legal;
- canonical fixtures require no migration;
- valid semantic/debug/egui behavior remains unchanged;
- no renderer, layout, form-schema, localization, or M27 scope is introduced.