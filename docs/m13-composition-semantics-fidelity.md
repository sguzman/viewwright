# M13 — Composition Semantics Fidelity

M13 makes the existing composition topology vocabulary truthful after resolution.

## Existing pressure

`CompositionSource.kind` and `CompositionSource.axis` are source strings, and `ResolvedComposition.kind` / `.axis` are also still strings. Validation currently accepts `row`, `column`, `stack`, `split`, and `overlay`, but the M4 layout engine only implements linear horizontal/vertical slot allocation. In practice:

- `row` has real horizontal semantics;
- `column` has real vertical semantics;
- `split` has real axis-driven linear semantics;
- `stack` and `overlay` do not have distinct implemented topology and currently fall through to ordinary linear layout.

That lets ViewWright claim semantics it does not actually project.

## M13 contract

M13 supports exactly the composition kinds whose layout semantics already exist:

- `split`
- `row`
- `column`

Resolution produces typed `CompositionKind` and typed `Axis` semantics. `stack` and `overlay` become validation errors until a future milestone earns and implements them.

`row` resolves to horizontal. `column` resolves to vertical. If source explicitly supplies a contradictory axis for either, validation fails rather than silently ignoring one declaration.

`split` remains axis-driven. Preserve the existing source compatibility for omitted split axis unless implementation evidence proves a stricter rule is already authoritative.

M13 adds no new TOML field and no intended visible redesign.