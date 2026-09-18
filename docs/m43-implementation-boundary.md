# M43 — Implementation Boundary

## In scope

- backward-compatible rich Document fixture schema;
- typed rich block model;
- document-local block validation;
- representative spoken range;
- fixture-aware concept/ASCII projection;
- egui rich document renderer;
- soft derived spoken-range paint using existing palette;
- new Lantern Leaf rich-document specimen;
- tests for validation/render/identity/exact verification;
- human QA.

## Likely crates

- `viewwright-model`;
- `viewwright-ascii`;
- `viewwright-concept`;
- `viewwright-egui`;
- `viewwright-preview`.

`viewwright-layout` should not require new geometry semantics.

`viewwright-expectation` and `viewwright-compare` should not change semantically.

## Out of scope

- new element kind;
- global/document-block author IDs;
- expectation 0.3;
- heading-specific source visual styling;
- font-family assets/binding;
- reading font vs UI font schema;
- column-width/page-margin document presentation;
- applying M42 control changes to renderer;
- multiple simultaneous highlights;
- sentence/word timing;
- TTS playback;
- auto-follow/auto-scroll behavior;
- EPUB parsing;
- annotation storage;
- assets/icons/covers;
- progress/seek;
- responsive variants;
- M44.

## Frozen specimens

Do not modify:

- M40 baseline;
- M41 furnished;
- M42 controls.

Add a separate M43 specimen.
