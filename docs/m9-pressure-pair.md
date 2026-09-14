# M9 — Density Pressure Pair

The repository contains two intentionally equivalent pressure specimens:

- `specimens/density-pressure-comfortable.toml`
- `specimens/density-pressure-dense.toml`

They share the same:

- purpose;
- regions and composition;
- spacing tokens;
- elements;
- command action;
- fixture content;
- collection presentation.

The required identity difference is the screen id; the only intended design-semantic difference is:

```text
comfortable  ↔  dense
```

Use fixture `populated` for both.

The pair exists to make density independently visible to human QA and regression tests.
