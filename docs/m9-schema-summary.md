# M9 — Schema Summary

M9 adds no new source field. It makes the existing screen-level `density` field semantically authoritative.

## Source

Supported forms:

```toml
[screen]
id = "project_browser"
purpose = "Browse projects and inspect the selected project"
density = "comfortable"
root = "workspace"
```

```toml
[screen]
id = "dependency_workbench"
purpose = "Inspect dependency health and package metadata"
density = "dense"
root = "workspace"
```

## Rules

- supported values are exactly `comfortable` and `dense`;
- omitted density resolves to `comfortable`;
- unknown values are rejected;
- density is screen-level only.

## Resolved shape

Conceptually:

```text
Density
  Comfortable
  Dense

ResolvedScreen
  id
  purpose
  density: Density
```

Exact Rust organization is an implementation choice, but backend code must consume typed resolved density rather than arbitrary strings.

## Backend policy

The source does not author concrete egui spacing values.

The egui backend maps resolved density into a small local micro-layout policy. Those concrete values remain backend implementation policy.

M9 adds no density metric table to TOML.
