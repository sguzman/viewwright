# M8 — Schema Summary

M8 adds no new source field. It makes the existing collection `presentation` field semantically authoritative.

## Source

Supported collection forms:

```toml
[[element]]
id = "navigation_items"
region = "navigation"
kind = "collection"
importance = "secondary"
presentation = "list"
```

```toml
[[element]]
id = "project_collection"
region = "projects"
kind = "collection"
importance = "primary"
presentation = "adaptive_cards"
```

## Rules

- `presentation` is valid only on `kind = "collection"`;
- supported values are exactly `list` and `adaptive_cards`;
- omitted collection presentation resolves to `list`;
- unknown values are rejected;
- non-collection use is rejected.

## Resolved shape

Conceptually:

```text
CollectionPresentation
  List
  AdaptiveCards

ResolvedElement
  ...
  presentation: Option<CollectionPresentation>
```

The exact field/type organization is an implementation choice, but successfully resolved presentation must be typed rather than an unchecked arbitrary string.

## Fixture content

No M8 fixture syntax is added.

Existing collection content remains:

```toml
[[fixture.content]]
element = "project_collection"
items = [
  { id = "project_a", label = "Project A · active" },
]
selected = "project_a"
```

Presentation belongs to the element; representative data belongs to the fixture.
