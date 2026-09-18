# M43 — Acceptance

## Compatibility

- every existing legacy document specimen parses/resolves unchanged;
- existing legacy document visual rendering remains unchanged;
- M40/M41/M42 Lantern Leaf specimens remain frozen.

## Model

- rich document blocks are typed;
- supported kinds are exactly eyebrow/heading/paragraph/quote/divider;
- local block IDs validate nonblank/unique;
- block-specific fields validate;
- legacy/rich forms are mutually exclusive;
- spoken range validates document-local block reference and Unicode-scalar offsets.

## Projection

- fixture-aware concept output exposes ordered rich blocks, IDs, kinds, heading level, and representative spoken range;
- add fixture-aware ASCII output or an equivalently inspectable deterministic text projection for rich document blocks while preserving existing `render(&blueprint)` APIs;
- block IDs are represented as local document identities, not global screen identities.

## egui

- rich block order renders faithfully;
- eyebrow/heading/paragraph/quote/divider are visibly distinguishable using existing visual vocabulary;
- no redundant generic document heading is rendered before the rich block sequence;
- one spoken range is softly highlighted;
- text before/inside/after the range remains exact;
- legacy document renderer remains visually unchanged.

## Identity

- document element author ID remains present;
- rich block IDs do not appear as M34 `author_id`;
- fixture-local range/block identity does not alter M33 expectation;
- element immediate authored-identity parent remains reader region.

## Verification

- expectation remains canonical 0.2;
- M35 algorithm unchanged;
- ViewWitness unchanged;
- all existing exact-verification cases remain green;
- new M43 rich-document Lantern Leaf specimen passes real FullOutput -> ViewWitness -> M35 exact comparison with zero gaps/mismatches.

## Lantern Leaf specimen

Add a new specimen; do not mutate M42.

Required rich document:

- eyebrow `CHAPTER 3`;
- heading level 1 `The Mountain Path`;
- divider;
- at least three identified paragraph blocks;
- one quote block;
- representative spoken range inside one paragraph.

Keep M42 controls/furnishing intact.

## Human QA

Human should verify:

- center now reads more like an authored book/chapter than title + anonymous paragraphs;
- chapter eyebrow, heading, divider, paragraphs, and quote are distinguishable;
- spoken range is visible and gentle rather than neon or selection-like;
- reader remains dominant;
- right inspector and TTS controls remain usable;
- scrolling/clipping regressions do not return;
- no new overlap/clipping.

No claim yet that font family, line height, margin, or other M42 settings actually re-style the reader.

## Regression

No visual-schema expansion, responsive work, EPUB runtime, TTS execution, or M44 work.
