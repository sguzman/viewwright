# M23 — Schema Summary

M23 adds no source syntax.

Existing required fields remain:

```text
collection item: id + label
tree node: id + label + optional parent
```

`label` must contain at least one non-whitespace character. Validation may inspect `trim()` only for blankness; valid authored label text is stored unchanged.

Labels remain display copy, not identity. Duplicate labels remain legal.