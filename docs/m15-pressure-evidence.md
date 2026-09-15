# M15 — Pressure Evidence

M15 is driven by existing accepted source and projection behavior.

## Resolved-model mismatch

Current `ResolvedBlueprint` stores:

```text
design: DesignSource
```

so the resolved model retains the source-layer representation directly.

## Existing validation

Resolution already checks `design.dominant` against both resolved region ids and resolved element ids. Invalid references fail before a blueprint is returned.

That means the resolver already knows the semantic target category but discards it by carrying the original string forward.

## Existing projection

The concept projector emits:

```text
Hierarchy
  dominant: <id>
```

so dominant is already treated as meaningful resolved design intent, not dead source metadata.

## Canonical authored values

Accepted canonical pressure uses region dominance:

- Project Browser: `projects`
- Reader Workspace: `reader`
- visual Reader Workspace: `reader`
- Dependency Workbench: `packages`
- Comfortable/Dense pressure pair: `content`

M15 therefore resolves existing intent more truthfully without creating a new feature surface.
