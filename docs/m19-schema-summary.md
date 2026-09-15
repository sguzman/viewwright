# M19 — Schema Summary

M19 adds no TOML syntax.

Existing source remains authoritative:

```toml
[tokens.color]
canvas = "#262B33"
accent = "#9BCBE0"
unused_valid = "#123456"
```

Every value in `tokens.color` must satisfy the existing six-digit hexadecimal color-literal parser, regardless of whether `[visual]` references that token.

The resolved model remains unchanged:

- `ResolvedVisual` continues to contain only the concrete palette roles consumed by the visual profile;
- unused source color tokens do not become resolved palette entries;
- missing visual references remain validation errors;
- valid source strings are interpreted exactly as before.

M19 changes validation coverage, not schema shape.