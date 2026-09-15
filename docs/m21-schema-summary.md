# M21 — Schema Summary

M21 adds no source fields and changes no resolved-model types.

Existing region sizing remains:

```toml
[[region]]
id = "sidebar"
width = "100px"
grow = 1
```

or, in a vertical parent:

```toml
[[region]]
id = "strip"
height = "50px"
grow = 1
```

Both declarations are already valid.

## Resolved semantics

`ResolvedRegion.width`, `ResolvedRegion.height`, and `ResolvedRegion.grow` remain unchanged.

M21 clarifies projection semantics only:

- relevant same-axis fixed size contributes the base allocation;
- positive `grow` contributes proportional remaining-space allocation;
- the two contributions are additive when both exist.

No mutual-exclusion rule is introduced.