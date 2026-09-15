# M12 — Schema Summary

M12 adds no TOML syntax.

Existing source remains:

```toml
[[region]]
id = "library"
role = "navigation"
importance = "secondary"
```

Resolution must map supported strings into a typed enum conceptually equivalent to:

```text
RegionRole
  Commands
  Controls
  Navigation
  PrimaryContent
  Inspector
  Status
```

Exact Rust naming is implementation-defined.

Unknown source values are validation errors. There is no default role because `role` is already a required authored field.

The source-facing spelling remains snake_case. Human-readable projections may render source-like names or typed debug names as long as output is deterministic and the ontology remains clear.
