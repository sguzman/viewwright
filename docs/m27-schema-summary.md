# M27 — Schema Summary

Existing source shape remains authoritative:

```text
DocumentSource {
    title: String,
    paragraphs: Vec<String>,
}
```

M27 adds no schema fields and no resolved types.

Validation adds one invariant:

`title.trim().is_empty()` must be false.

The authored `title` string itself is not modified. `paragraphs` retain their existing semantics and validation behavior.
