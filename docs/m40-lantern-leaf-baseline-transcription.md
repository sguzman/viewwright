# M40 — Lantern Leaf v0.1-language baseline transcription

## High-level goal

Create an honest ViewWright transcription of the Lantern Leaf EPUB + TTS reader north star using only the accepted v0.1 language.

M40 is a pressure-capture milestone, not a schema expansion.

## Why first

The capability audit is based on source inspection. M40 makes the gap executable and visible.

A real baseline will show:

- which major structures map cleanly;
- where local furnishing has to be flattened;
- where fake regions are required to simulate local grouping;
- what the current concept specification omits;
- what the egui preview can and cannot communicate;
- whether the existing expectation/ViewWitness chain remains useful on a more sophisticated specimen.

That evidence should drive the first v0.2 language expansion.

## Reference

North-star evidence comes from:

- `sguzman/lantern-leaf/docs/concept-art/epub-tts-reader/README.md`;
- `sguzman/lantern-leaf/docs/concept-art/epub-tts-reader/furnishing.toml`;
- `sguzman/lantern-leaf/docs/concept-art/epub-tts-reader/leafline-epub-tts-reader-concept.webp`.

Do not import Lantern Leaf code or make it a repository dependency.

## Required baseline

Add a ViewWright specimen under `specimens/` representing the best honest current-model approximation of:

- library/navigation rail;
- dominant reader;
- inspector;
- persistent TTS surface;
- representative navigation/TOC content;
- reader search/document/status content;
- representative settings information;
- representative TTS commands/status.

Use the Lantern Leaf palette as closely as current semantic roles allow.

Use current fixed/grow geometry to approximate the 260 / grow / 330 shell and 96 px TTS surface.

Use only existing 0.1 schema.

## Required findings document

Produce a companion v0.2 baseline findings document that records every important compromise.

At minimum classify:

- direct mapping;
- approximation;
- fake-region inflation;
- flattened furnishing;
- missing control semantics;
- missing document semantics;
- missing visual semantics;
- missing representative state;
- missing responsive behavior;
- concept-projection loss.

Do not hide gaps by choosing a simpler interpretation of the north star.

## Projection evidence

Exercise the baseline through the existing accepted pipeline:

- parse / validate / resolve;
- semantic/debug projection;
- ASCII;
- concept specification;
- LayoutPlan;
- egui preview;
- M33 expectation;
- M37 ViewWitness conversion;
- M35 exact comparison where the existing model has shared evidence.

M40 must not weaken any existing verification contract.

## Human QA

Human review should compare the preview against the archived concept at the level of:

- recognizable shell;
- dominant reader;
- relative supporting surfaces;
- obvious missing furnishing/detail.

The acceptance question is **not** "does it look finished?"

The acceptance question is whether the baseline honestly exposes what 0.1 can and cannot author.

## Forbidden

M40 must not add:

- new element kinds;
- new fixture-state types;
- nested furnishing schema;
- responsive schema;
- new visual token roles;
- font-family support;
- asset/icon schema;
- renderer special cases for Lantern Leaf;
- hard-coded IDs;
- application runtime behavior;
- M41 implementation.

If the current language cannot express something, document the gap.

## Stop condition

Stop once one canonical baseline specimen and its evidence establish the real post-0.1 pressure surface.

The expected next pressure is nested semantic furnishing/local layout, but M40 must report what the baseline actually demonstrates rather than forcing that conclusion.
