# M34 — Schema Summary

M34 adds no authored TOML fields and no resolved-model fields.

It adds an egui/AccessKit projection contract only.

Conceptual observed accessibility hierarchy:

```text
AccessKit node
  author_id = <screen.id>
  └─ region anchor
       author_id = <region.id>
       └─ element anchor
            author_id = <element.id>
            └─ ordinary egui accessibility nodes
```

## Screen identity

Exact authored `screen.id`.

## Region identity

Exact authored/resolved region ID.

Region anchor bounds should correspond to the region's already-planned rectangle.

## Element identity

Exact authored/resolved element ID.

Element anchors identify the semantic ViewWright element as a whole; they do not create a new ViewWright geometry model.

## Explicit absences

No author-ID schema for fixture collection items, tree nodes, document paragraphs, property rows, or other fixture-local subcontent.

No comparison result, tolerance, ViewWitness witness schema, responsive rule, or new TOML syntax.