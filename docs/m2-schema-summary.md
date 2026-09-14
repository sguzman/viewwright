# M2 Schema Summary

The visual-authoring pressure test proposes only the following canonical additions:

```text
tokens.color
  named semantic colors

tokens.type
  display / heading / body / caption sizes

visual
  semantic palette bindings
  corner token reference
  border policy

region.surface
  canvas | panel | raised | transparent
```

Everything else in M2 should be derived from existing semantics:

- region role
- element kind
- importance
- spacing tokens
- design character
- avoidances

The proposal deliberately does **not** add arbitrary element style blocks, selectors, CSS classes, theme inheritance, or backend widget properties.

The concept-specification projection and egui renderer should both consume the same resolved visual model.
