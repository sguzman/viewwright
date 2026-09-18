# M43 — Pressure Evidence

## Current accepted north-star state

M42 makes recurring settings truthful:

- choice;
- boolean;
- scalar;
- typed fixture values;
- typed host value-change reports.

M41 makes local interior structure truthful.

The center reader remains one of the least expressive surfaces.

## Current ViewWright document model

Current source:

```rust
DocumentSource {
    title: String,
    paragraphs: Vec<String>,
}
```

Current renderer:

- label title;
- iterate anonymous paragraphs;
- no block taxonomy;
- no block ID;
- no inline representative state.

## Lantern Leaf pressure

External north-star document explicitly contains:

- eyebrow;
- heading level;
- divider;
- identified paragraphs `p1`, `p2`, `p3`;
- quote;
- current TTS block;
- current spoken range;
- soft sentence highlight.

The product intention says reading and listening are one activity and the spoken sentence belongs visibly inside the document.

## Why block IDs remain fixture-local

Existing ViewWright intentionally separates fixture-local collection/tree item IDs from M34 authored screen/region/element IDs.

Rich document blocks are fixture data in M43.

Promoting them to global author IDs would be a different identity-model milestone and would force expectation/comparator decisions not required to make the reader semantically richer.

M43 therefore gives blocks stable document-local identity without changing M34.

## Why not typography first

M42's controls expose font/size/spacing intent, but the host still owns their effects.

Before ViewWright authors richer reading presentation, it needs a truthful document hierarchy to present.

## Why not responsive now

Responsive authoring remains mandatory for v0.2, but varying a reader that still consists of anonymous paragraph strings would not address the dominant current semantic gap.
