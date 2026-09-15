# M18 Schema Summary

M18 changes no source syntax and introduces no new resolved types.

Existing source remains:

```text
screen.root -> composition id
composition.children -> region/composition ids
```

Existing resolved composition representation remains authoritative.

The only schema-level semantic change is a stronger validation invariant:

> Every declared composition must be acyclic, regardless of reachability from `screen.root`.

Unused but acyclic declarations remain legal.
