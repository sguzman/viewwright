# M15 — Schema Summary

M15 changes no authored TOML syntax.

## Source

Existing source remains:

```text
DesignSource
  character: Vec<String>
  dominant: Option<String>
  avoid: Vec<String>
```

`dominant` remains optional and source-facing.

## Resolved

Introduce a resolved design representation conceptually equivalent to:

```text
ResolvedDesign
  character: Vec<String>
  dominant: Option<DominantTarget>
  avoid: Vec<String>

DominantTarget
  Region(String)
  Element(String)
```

Exact field and variant naming is implementation detail.

## Mapping

- authored region id → `DominantTarget::Region`
- authored element id → `DominantTarget::Element`
- omitted value → `None`
- every other authored identifier/value → validation error

The target id should remain source-like and deterministic when projected.

M15 does not require introducing general RegionId/ElementId newtypes across the whole model.
