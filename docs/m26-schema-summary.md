# M26 — Schema Summary

Source shape remains unchanged:

```text
PropertySource {
  name: String,
  value: String,
}
```

Resolved shape remains unchanged:

```text
ResolvedProperty {
  name: String,
  value: String,
}
```

M26 adds only source validation:

- `name.trim().is_empty()` => validation error;
- valid names copied unchanged;
- `value` receives no new validation.

No new property identifiers, newtypes, optionality, schema metadata, or renderer-facing structures are added.