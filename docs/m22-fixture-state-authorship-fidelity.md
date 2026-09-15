# M22 — Fixture State Authorship Fidelity

Fixtures are canonical representative preview/test states. `fixture.state` is already a required authored field, survives resolution, and appears in semantic/debug inspection, but resolution currently accepts empty or whitespace-only state text.

## Principle

A required representative state must actually describe a state.

`fixture.state` must contain at least one non-whitespace character. Validation may inspect `state.trim().is_empty()`, but valid state text must be preserved exactly as authored.

## Contract

Reject:

```toml
[[fixture]]
id = "empty_state"
state = ""
```

and whitespace-only equivalents.

Preserve nonblank authored state text byte/string-equivalently, including intentional leading/trailing whitespace.

`FixtureSource.state` and `ResolvedFixture.state` remain required `String` values. Missing `state` continues to fail TOML deserialization.

## Boundary

M22 does not add state enums, lifecycle semantics, transitions, state machines, normalization, generated defaults, localization, general fixture-content validation, or general copy sanitation.