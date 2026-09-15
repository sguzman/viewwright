# M15 — Dominant Target Fidelity

ViewWright already authors explicit hierarchy intent through `design.dominant`.

Accepted canonical screens use it to name the region intended to dominate the composition, and the concept projection exposes that value as part of the hierarchy specification. Resolution already validates that the authored identifier refers to either a region or an element.

The current resolved model nevertheless stores the original `DesignSource` wholesale:

```text
SourceBlueprint.design: DesignSource
        ↓ validation of dominant reference
ResolvedBlueprint.design: DesignSource
```

That means resolution proves the reference but then discards what kind of semantic target it resolved to. Downstream consumers receive an arbitrary string again.

## Principle

Resolved references should preserve the semantic identity established during validation.

M15 introduces a resolved design layer conceptually equivalent to:

```text
ResolvedDesign
  character: Vec<String>
  dominant: Option<DominantTarget>
  avoid: Vec<String>

DominantTarget
  Region(id)
  Element(id)
```

Exact Rust naming is implementation detail.

## Existing source contract

No TOML syntax changes.

Existing authoring remains:

```toml
[design]
character = ["quiet", "focused"]
dominant = "reader"
avoid = ["toolbar_dominance"]
```

`dominant` remains optional.

The existing accepted source contract that a dominant target may refer to a region or an element is preserved. M15 does not widen that contract to compositions, fixtures, actions, or arbitrary strings.

## Canonical pressure

Accepted canonical screens currently use region dominance:

- Project Browser → `projects`
- Reader Workspace → `reader`
- visual Reader Workspace → `reader`
- Dependency Workbench → `packages`
- density pressure pair → `content`

M15 therefore has no intended visible effect.

## Boundary

M15 is resolution fidelity, not a visual-dominance renderer feature.

It does not add special styling, sizing, focus, layout priority, z-order, navigation, accessibility semantics, responsive behavior, or new authoring syntax.

The egui backend remains unchanged unless a compile-driven type update is mechanically necessary; it must not begin interpreting `design.dominant` as new runtime behavior.
