# M13 — Schema Summary

M13 adds no TOML syntax.

Existing source remains:

```toml
[[composition]]
id = "workspace"
kind = "column"
axis = "vertical"
children = ["app_commands", "reading_body", "transport"]
```

Resolution maps source strings into typed semantics conceptually equivalent to:

```text
CompositionKind
  Split
  Row
  Column

Axis
  Horizontal
  Vertical
```

Source spellings remain lowercase snake_case.

Compatibility rules:

- `row` implies Horizontal;
- `column` implies Vertical;
- an explicit contradictory axis on `row` or `column` is invalid;
- `split` uses authored axis when present and preserves the current resolver's existing omitted-axis compatibility behavior;
- `stack` and `overlay` are invalid in M13 because ViewWright does not yet implement distinct topology for them.

Exact Rust naming is implementation-defined.