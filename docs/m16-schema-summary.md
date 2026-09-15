# M16 — Schema Summary

M16 adds no source syntax.

Existing composition source remains:

```toml
[[composition]]
id = "workspace"
kind = "column"
children = ["commands", "body", "status"]
```

The change is validation of existing child relationships.

## Ownership contract

Across all `composition.children` relationships:

- each region id has at most one parent composition;
- each non-root composition id has at most one parent composition;
- `screen.root` has zero parents;
- one parent may not list the same child id more than once.

Unreferenced declarations remain outside M16's scope. No parent field is added to source or resolved structures.