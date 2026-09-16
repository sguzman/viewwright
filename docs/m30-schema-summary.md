# M30 — Schema Summary

M30 does not change source syntax or public resolved-model shape.

Existing source remains:

```toml
[[fixture.content]]
element = "some_element"
# exactly one existing payload family
```

Existing element declarations remain authoritative for semantic kind and owning region.

M30 adds one resolution-time cross-field invariant:

```text
fixture.content.element
    -> existing element
    -> element.region
    -> region must be reachable from validated screen.root
```

No new fields, wrapper types, IDs, aliases, path objects, or reachability metadata are serialized.

Unused declarations remain legal generally. Only an element explicitly targeted by fixture content must be root-reachable.