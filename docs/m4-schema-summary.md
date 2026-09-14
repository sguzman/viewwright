# M4 — Schema Summary

M4 adds exactly one authored field.

## Composition growth

A composition may declare:

```toml
[[composition]]
id = "body"
kind = "split"
axis = "horizontal"
children = ["filters", "packages", "inspector"]
gap = "md"
grow = 1
```

`grow` is a non-negative finite numeric weight.

Default: `0`.

It matters only when the composition itself is a child of another composition and participates in that parent's main-axis allocation.

## Existing region sizing remains unchanged

Regions continue to support:

- `width = "…px"`
- `height = "…px"`
- `grow = <weight>`

M4 should validate region/composition growth consistently.

## No new source syntax for layout plans

Layout rectangles are derived projection output, not authored TOML.

The layout projection consumes:

- resolved structure
- resolved fixed sizes/grow
- resolved spacing
- a concrete viewport size

and produces backend-independent rectangles.

## Explicit non-additions

M4 does not add:

- percentages
- min/max/clamp
- breakpoints
- alignment fields
- positioning coordinates
- margins
- per-child overrides
- grid tracks
- flexbox-like property families
- element-level geometry

Any of those require future specimen pressure.