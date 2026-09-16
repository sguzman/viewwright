# M32 — Schema Summary

M32 adds one optional field to authored regions:

```toml
[[region]]
id = "reader"
role = "primary_content"
importance = "primary"
grow = 1
overflow = "scroll_y"
```

Supported values:

- `clip`
- `scroll_y`

Omitted `overflow` defaults to `clip`.

The resolved region model should carry a typed overflow policy conceptually equivalent to:

```text
OverflowPolicy::Clip
OverflowPolicy::ScrollY
```

Exact Rust naming is implementation detail.

No new fields are added to elements, fixtures, compositions, screen, visual tokens, or LayoutPlan.

No source syntax is added for scroll position, horizontal scrolling, scrollbar style, page size, anchors, virtualization, or responsive breakpoints.
