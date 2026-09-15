# M17 Pressure Evidence — Structural Identifier Fidelity

## Existing architectural pressure

`docs/model.md` states that every projection should preserve stable semantic IDs wherever feasible and that validation determines whether authored documents are coherent before resolution/projection.

General structural IDs are currently plain strings with duplicate/reference validation but no nonblank requirement.

## Concrete failure

Current root resolution uses an empty string as an internal failure sentinel:

```rust
let root = match source.screen.root.as_deref() {
    None => {
        errors.push(...);
        String::new()
    }
    Some(root) if !composition_ids.contains(root) => {
        errors.push(...);
        String::new()
    }
    Some(root) => root.to_owned(),
};
```

Later root-dependent validation is guarded by `if !root.is_empty()`.

Because `composition.id = ""` is currently legal, `screen.root = ""` can successfully name an authored composition while also looking like the resolver's invalid-root sentinel. That creates a source/internal-state collision and skips root-dependent checks.

## Local identity pressure

Collection item IDs are addressed by `selected`; tree node IDs are addressed by `selected` and `parent`. Their duplicate relationships are already validated, so they are genuine local identity rather than arbitrary payload text. Blank/whitespace-only local IDs undermine those relationships in the same way.

## Why this earns a milestone

M17 does not invent a naming convention. It closes a correctness hole where authored identity can collide with resolver control state and makes the existing stable-ID contract explicit across both global structural IDs and local fixture identities.