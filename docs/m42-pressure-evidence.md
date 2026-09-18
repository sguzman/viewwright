# M42 — Pressure Evidence

## M41 removed structural ambiguity

After M41, the north-star screen has truthful region-local groupings.

The remaining inspector problem is no longer "where can these things go?"

It is "what are these things?"

## Furnished specimen evidence

The M41 specimen still represents:

- Text and layout settings as one PropertySheet;
- Theme/annotations as one PropertySheet;
- Speech settings as one PropertySheet;
- Voice and Speed as Commands.

Those types are knowingly false/approximate.

## External north-star frequency

Lantern Leaf repeatedly uses:

### finite choices

- font family;
- text alignment;
- reading flow;
- theme-like selections;
- voice;
- TTS rate.

### booleans

- dyslexia-friendly font;
- show highlights;
- show notes;
- show bookmarks;
- follow reading/highlighting flags.

### bounded scalars

- font size;
- line height;
- paragraph spacing;
- page margin;
- column width;
- brightness;
- contrast;
- volume.

These are recurring semantic families, not one-off widget trivia.

## Why not color/progress/tabs yet

Color values have additional visual semantics.

Progress/seek introduces temporal position and optional seekability.

Tabs imply conditional content/view selection.

Each deserves separate pressure rather than being forced into a generic choice/value abstraction.

## Why typed fixture state now

A control without a representative value cannot produce an honest isolated preview.

String property rows are exactly the approximation M42 is replacing.

## Why typed host change reports

The charter already says the render thread may detect lightweight interactions and enqueue/report work while the host owns application logic.

Commands follow this pattern today.

Semantic value controls should preserve that boundary rather than either becoming decorative or executing business logic inside ViewWright.
