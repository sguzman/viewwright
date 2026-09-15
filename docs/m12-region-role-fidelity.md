# M12 — Region Role Fidelity

M12 makes the existing authored `region.role` field a typed, validated semantic contract.

## Existing pressure

`RegionSource.role` is currently a `String`, `ResolvedRegion.role` remains a `String`, and resolution copies the value without validation. The egui backend already branches on the literal role string `"commands"` to choose horizontal command layout, while semantic/debug and concept projections expose role as ontology.

This creates an honesty gap: a typo such as `commmands` silently degrades downstream behavior instead of failing validation.

## Canonical vocabulary

M12 supports exactly the region roles already exercised by accepted canonical specimens:

- `commands`
- `controls`
- `navigation`
- `primary_content`
- `inspector`
- `status`

## Intended pipeline

```text
authored region.role
    ↓
validated typed RegionRole
    ↓
ResolvedRegion.role
    ├── semantic/debug
    ├── concept
    └── egui role-specific projection
```

## Design rule

Region roles are semantic ontology, not arbitrary decoration and not product-facing labels.

M12 adds no new source field and no new visible chrome. It makes an existing field truthful and machine-checkable.
