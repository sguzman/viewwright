# M14 — Schema Summary

M14 adds no TOML field.

Existing valid source remains:

```toml
[[element]]
id = "project_search"
region = "navigation"
kind = "search"
importance = "secondary"
label = "Search projects"
```

The `label` field becomes semantically required for every current element kind.

Source-facing shape may remain:

```text
ElementSource.label: Option<String>
```

so the validator can report missing-label errors in author terms.

Successful resolution remains conceptually:

```text
ResolvedElement.label: String
```

with the stronger invariant that the string came from authored `label` and is not empty after trimming.

Invalid:

```toml
[[element]]
id = "project_search"
region = "navigation"
kind = "search"
importance = "secondary"
```

Invalid:

```toml
label = "   "
```

M14 does not introduce inferred labels, aliases, defaults, placeholders, or hidden-label syntax.