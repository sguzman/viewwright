# M22 — Acceptance

M22 is accepted when required fixture state text cannot be semantically blank.

## Required validation

- reject `fixture.state = ""`;
- reject whitespace-only fixture state;
- preserve valid state text exactly;
- preserve missing-field TOML deserialization failure;
- keep `ResolvedFixture.state` as `String`.

## Compatibility

Accepted canonical fixtures require no migration and valid semantic/debug output remains unchanged.

## Tests

Cover empty state, whitespace-only state, exact valid-state preservation, missing state, and canonical compatibility.

## Boundary

No state enum, grammar, transition model, normalization, generated fallback, or general copy-validation work.