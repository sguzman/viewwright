# M31 — Schema Summary

M31 adds no new TOML field.

It re-admits one composition-kind value with a deliberately narrow contract:

```toml
[[composition]]
id = "root_overlay"
kind = "overlay"
children = ["workspace", "palette_surface"]
```

For `overlay`:

- `children` contains exactly two authored references;
- child 0 resolves to a composition;
- child 1 resolves to a region;
- `axis` is omitted;
- `gap` is omitted;
- `padding` remains optional under existing token-reference semantics;
- the floating region must author both `width` and `height`;
- the floating region may not use positive `grow` as overlay-parent sizing intent.

Linear composition syntax remains unchanged:

- `split` is axis-driven;
- `row` is horizontal;
- `column` is vertical.

`stack` remains invalid.

Resolved composition topology must distinguish axisless overlay from axis-bearing linear kinds. Exact Rust representation is implementation detail, but a fake axis is not acceptable.

No new region role, element kind, fixture family, visual token, alignment field, anchor field, coordinate field, z-index field, or visibility field is introduced.
