# M5 — Schema Summary

M5 adds only the fixture-content vocabulary needed to make the Reader Workspace representative without backend fiction.

## Existing fixture-content families

M3 already supports:

- `items` + optional `selected` for `collection`
- `properties` for `property_sheet`
- `text` for `status`

These remain unchanged.

## New tree family

```toml
[[fixture.content]]
element = "document_outline"
nodes = [
  { id = "intro", label = "Introduction" },
  { id = "part_one", label = "Part I" },
  { id = "chapter_one", label = "Chapter 1", parent = "part_one" },
]
selected = "chapter_one"
```

Node fields:

- `id: string`
- `label: string`
- `parent: optional string`

Tree compatibility:

- only `tree` elements accept `nodes`
- `selected` references a node id when `nodes` is the active payload family

## New document family

```toml
[[fixture.content]]
element = "document_surface"
document = { title = "The Quiet Machine", paragraphs = ["First paragraph.", "Second paragraph."] }
```

Document fields:

- `title: string`
- `paragraphs: ordered array of strings`

Document compatibility:

- only `document` elements accept `document`

## Payload-family exclusivity

A fixture-content record must contain exactly one semantic payload family:

- `items`
- `properties`
- `text`
- `nodes`
- `document`

`selected` is auxiliary and is valid only with collection/tree payloads.

## Resolved representation

The resolved model should use typed variants rather than arbitrary maps, conceptually:

```text
ResolvedFixtureContent::Tree {
    element,
    nodes,
    selected,
}

ResolvedFixtureContent::Document {
    element,
    title,
    paragraphs,
}
```

Tree nodes retain stable semantic ids and validated parent references.

## Explicit omissions

No schema is added for:

- expanded/collapsed nodes
- tree icons/check states
- document blocks beyond plain paragraphs
- rich text spans
- images
- links
- annotations
- pagination
- runtime actions
- editable values
