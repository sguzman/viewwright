# M41 — Schema Summary

## Region addition

A region may name one local furnishing root:

```toml
[[region]]
id = "reader"
role = "primary_content"
importance = "primary"
surface = "panel"
furnishing = "reader_furnishing"
```

The exact field is `furnishing`.

Omission preserves the existing flat-element behavior.

## Furnishing declaration

M41 adds:

```toml
[[furnishing]]
id = "reader_furnishing"
kind = "column"
children = ["toolbar_slot", "document_slot"]
gap = "sm"
padding = "sm"
overflow = "clip"

[[furnishing]]
id = "toolbar_slot"
kind = "row"
children = ["search_slot", "toolbar_actions"]
height = "52px"
gap = "md"

[[furnishing]]
id = "search_slot"
kind = "row"
children = ["book_search"]
grow = 1

[[furnishing]]
id = "toolbar_actions"
kind = "row"
children = ["previous_chapter", "next_chapter", "chapter_status", "bookmark_chapter"]
width = "520px"
gap = "sm"

[[furnishing]]
id = "document_slot"
kind = "column"
children = ["chapter_document"]
grow = 1
overflow = "scroll_y"
```

Fields:

- `id: String`
- `kind: "row" | "column"`
- `children: [String, ...]`
- optional `gap`: existing spacing-token reference
- optional `padding`: existing spacing-token reference
- optional `width = "<n>px"`
- optional `height = "<n>px"`
- optional `grow = nonnegative finite number`
- optional `overflow = "clip" | "scroll_y"`, default `clip`

No surface, color, role, importance, label, action, or application state belongs on a furnishing.

## Typed resolved model

Add a typed `ResolvedFurnishing` with:

- stable structural id;
- `FurnishingKind::{Row, Column}`;
- ordered typed children:
  - furnishing reference;
  - element reference;
- resolved numeric gap/padding;
- optional fixed width/height;
- grow;
- existing typed overflow policy.

`ResolvedRegion` gains an optional furnishing-root ID.

`ResolvedBlueprint` gains resolved furnishings.

## Child-type rule

A furnishing's direct children are either:

- all furnishings; or
- all elements.

Mixed child classes are invalid in M41.

## Ownership and reachability

For each furnished region:

- the root must resolve to a furnishing;
- the root cannot be parented by another furnishing;
- all reachable element children must have `element.region == region.id`;
- every element owned by the region must be reachable exactly once;
- furnishing children have unique structural parents;
- element children have unique furnishing parents;
- furnishing cycles are rejected;
- every authored furnishing must be reachable from exactly one region furnishing root in M41.

## LayoutPlan

`LayoutPlan` gains deterministic furnishing rectangles and:

```rust
plan.furnishing(id)
```

The furnishing root receives its region rectangle.

Branch children are fixed-plus-grow allocated on the furnishing axis.

Leaf furnishings receive a rectangle but do not create backend-independent element rectangles.

## Expectation / observation schema

No change.

M33 expectation 0.1 remains region + element based.

Furnishing IDs are not M34 author IDs in M41.
