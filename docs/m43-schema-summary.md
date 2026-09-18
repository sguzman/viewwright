# M43 — Schema Summary

## Existing legacy document form

Unchanged:

```toml
document = {
  title = "The Mountain Path",
  paragraphs = ["...", "..."]
}
```

## Rich document form

```toml
document = {
  blocks = [
    { id = "chapter_3", kind = "eyebrow", text = "CHAPTER 3" },
    { id = "chapter_title", kind = "heading", level = 1, text = "The Mountain Path" },
    { id = "opening_divider", kind = "divider" },
    { id = "p1", kind = "paragraph", text = "..." },
    { id = "p2", kind = "paragraph", text = "..." },
    { id = "closing_quote", kind = "quote", text = "..." }
  ],
  spoken = { block = "p2", start = 0, end = 104 }
}
```

## Document form exclusivity

A document must be exactly one of:

1. legacy form with both nonblank `title` and nonempty `paragraphs`;
2. rich form with nonempty `blocks`.

Reject:

- title without paragraphs;
- paragraphs without title;
- legacy title/paragraphs mixed with blocks;
- empty rich blocks.

`spoken` is valid only on the rich form.

## Document block source

Fields:

- `id: String`;
- `kind: eyebrow | heading | paragraph | quote | divider`;
- optional `text`;
- optional integer `level`.

Validation:

- ID nonblank and unique within the document;
- textual blocks require nonblank text;
- heading requires `level` in 1..=6;
- non-heading blocks reject `level`;
- divider rejects text and level.

## Spoken range

Fields:

- `block: String`;
- `start: usize`;
- `end: usize`.

Validation:

- block exists;
- target is textual;
- start < end;
- offsets are within Unicode scalar-value length.

## Resolved model

Represent legacy and rich documents as an explicit typed enum rather than sentinel/empty fields.

Conceptually:

```rust
ResolvedDocument::Legacy {
    title,
    paragraphs,
}

ResolvedDocument::Rich {
    blocks: Vec<ResolvedDocumentBlock>,
    spoken: Option<ResolvedSpokenRange>,
}
```

Resolved rich block kind is typed.

Do not erase heading level or local block ID.

## Fixture content

`ResolvedFixtureContent::Document` should contain one resolved document value rather than parallel title/paragraph fields.

Existing caller behavior must remain source-compatible at TOML level.

## Expectation

No expectation version change.

Canonical expectation remains 0.2.

Reason:

- semantic element kind remains `document`;
- rich block identity/state is fixture-local evidence, not exported M33 author identity.

M35 remains unchanged.
