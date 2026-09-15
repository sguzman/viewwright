# M17 Schema Summary — Structural Identifier Fidelity

M17 adds no TOML fields and no new public schema types.

Existing source IDs remain strings.

Validation now requires these authored identifiers to contain at least one non-whitespace character:

- `screen.id`
- `region.id`
- `element.id`
- `composition.id`
- `fixture.id`
- collection item `id`
- tree node `id`

Valid IDs are preserved byte-for-byte as authored.

Reference fields keep exact-string semantics.

`ResolvedBlueprint.root` may remain `String`, but resolution must represent invalid/missing root state explicitly during validation rather than using an authored-empty-compatible `String::new()` sentinel.

No identifier normalization, generated IDs, global newtype migration, or new namespace rules are part of M17.