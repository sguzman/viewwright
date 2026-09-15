# M22 — Schema Summary

M22 adds no source fields and no resolved types.

Existing source:

```toml
[[fixture]]
id = "healthy"
state = "ready"
```

remains unchanged.

The only contract change is validation: `fixture.state` must contain at least one non-whitespace character.

Validation does not trim or normalize accepted state text. `ResolvedFixture.state` remains the exact authored `String`.