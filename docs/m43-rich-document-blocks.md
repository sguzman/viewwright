# M43 — Rich Document Blocks and Spoken-Range Projection

## Pressure

After M42, Lantern Leaf's shell, local furnishing, and recurring settings controls are structurally and semantically credible.

The dominant reader is not.

Current ViewWright document fixture content is:

```text
title
paragraphs: Vec<String>
```

That loses:

- chapter eyebrow;
- heading level;
- paragraph identity;
- quote semantics;
- divider semantics;
- stable local block references;
- representative current spoken text.

Lantern Leaf's north star explicitly models those distinctions.

M43 enriches the document fixture model without turning ViewWright into an EPUB parser or rich-text engine.

## Two document forms

Existing simple documents remain valid exactly as authored today.

M43 adds a second rich form.

### legacy simple form

```toml
document = {
  title = "The Mountain Path",
  paragraphs = ["...", "..."]
}
```

Existing behavior remains unchanged.

### rich block form

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

The exact TOML may be multiline rather than one literal line, but the schema is canonical as described.

## Block kinds

M43 supports exactly:

- `eyebrow`;
- `heading`;
- `paragraph`;
- `quote`;
- `divider`.

No arbitrary rich-text/span tree.

### Textual blocks

Eyebrow, heading, paragraph, and quote require nonblank text.

Heading additionally requires integer `level` from 1 through 6.

Non-heading textual blocks must not carry `level`.

### Divider

Divider carries:

- ID;
- kind.

It must not carry text or heading level in M43.

Width/style remain backend presentation, not source syntax yet.

## Local block identity

Every rich block has a nonblank ID unique within that document fixture content.

Block IDs are **document-local fixture identity**.

They are:

- stable references for representative spoken state;
- visible in fixture-aware projections;
- deterministic across rendering of the same fixture.

They are not:

- global ViewWright declaration IDs;
- M34 AccessKit `author_id`;
- M33/M42 expectation element IDs.

This follows the existing separation between authored screen/region/element identity and fixture-local collection/tree data.

## Representative spoken range

A rich document may author zero or one representative spoken range:

```toml
spoken = { block = "p2", start = 0, end = 104 }
```

Semantics:

- `block` must resolve to a textual block in the same document;
- `start` is inclusive;
- `end` is exclusive;
- offsets count Unicode scalar values, not UTF-8 bytes;
- `start < end`;
- `end` must not exceed the textual block's scalar length.

Divider cannot be spoken.

M43 does not author sentence segmentation, word timing, audio timestamps, playback state, auto-scroll, or follow policy.

The range is representative fixture state only.

## Renderer

Legacy simple documents preserve their existing rendering behavior.

Rich documents render block order faithfully.

Expected semantic projection:

- eyebrow: compact/muted chapter metadata using existing visual vocabulary;
- heading: existing heading scale / strong text;
- paragraph: existing body text;
- quote: distinguishable literary text using existing palette/type capabilities, without new source visual roles;
- divider: restrained separator from existing border color when visual authoring exists.

The rich path should not display a redundant generic document label above an authored eyebrow/heading sequence.

The document element remains the M34 semantic author anchor.

Block IDs remain local data and must not become author IDs.

## Spoken-range visual projection

The representative spoken range should be visibly projected into the textual block.

M43 may derive a restrained soft fill from existing authored `surface` / `surface_raised` / `accent` colors.

Do not add a new palette field in M43.

The highlight should:

- be visible;
- remain calm enough for long-form reading;
- preserve readable text;
- affect only the authored range;
- not look like text selection or a full control button.

The exact richer highlight role remains Stage 4 visual-authorship pressure.

## Runtime boundary

M43 does not:

- play audio;
- advance spoken offsets;
- auto-scroll;
- follow playback;
- parse EPUB;
- bind TTS engine state;
- persist annotations.

A host may later update fixtures/runtime state using separate APIs.

## Presentation boundary

M43 does not make M42 settings change the document renderer.

Font Family, Font Size, Line Height, Paragraph Spacing, Page Margin, Column Width, Text Alignment, and Reading Flow remain representative controls whose application effects are host-owned.

Static literary typography and richer document presentation remain further Stage 3 / Stage 4 pressure.

## Why this comes before visual refinement

The current reader needs semantic blocks before typography, highlight roles, assets, or concept-art fidelity can be authored truthfully.

Styling anonymous paragraph strings more elaborately would preserve the wrong model.
