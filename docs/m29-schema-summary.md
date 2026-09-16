# M29 — Schema Summary

M29 adds no TOML fields and changes no source types.

Existing source remains:

```toml
[design]
dominant = "reader"
```

Existing resolved types remain:

```text
ResolvedDesign
  dominant: Option<DominantTarget>

DominantTarget
  Region(id)
  Element(id)
```

The only new rule is cross-structure validation:

- `Region(id)` is valid only if that region occurs in the composition tree reachable from `screen.root`;
- `Element(id)` is valid only if the element's authored owning region occurs in that reachable region set.

No reachability information is serialized into a new public wrapper or token. No source normalization is introduced.